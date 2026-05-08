# Inspecting Rust Code with MIR

Notes on how to look "under the hood" of your code — what's a copy vs move, where drops run, what autoderef expands to.

## The compilation pipeline

```
Source (.rs)
   ↓
HIR    — method resolution, autoref/autoderef
   ↓
THIR   — pattern matching desugared
   ↓
MIR    — borrow check, drops explicit, CFG form   ← sweet spot for learning
   ↓
LLVM IR — optimizations, inlining
   ↓
Machine code
```

## Dumping MIR

**Stable (one final snapshot):**
```bash
cargo rustc --lib -- --emit=mir
# output in target/debug/deps/*.mir
```

**Nightly (richer, per-pass, filterable):**
```bash
cargo +nightly rustc --lib -- -Z unpretty=mir 2>/dev/null > arc.mir
cargo +nightly rustc --lib -- -Z dump-mir=all       # ./mir_dump/
cargo +nightly rustc --lib -- -Z dump-mir-graphviz  # CFG as .dot
```

`-Z` flags require nightly because they're unstable compiler internals.

## Reading MIR — vocabulary

| Symbol | Meaning |
|---|---|
| `_0` | Return value |
| `_1`, `_2`, … | First arg, second arg, then locals |
| `bb0`, `bb1`, … | Basic blocks (straight-line code chunks) |
| `*_3` | Dereference local 3 |
| `(_3.0: T)` | Field 0 of `_3`, with type T (MIR uses indices, not field names) |
| `copy _1` | Bitwise copy — type is `Copy`, source still usable |
| `move _1` | Ownership transfer — source no longer valid |
| `const ZeroSized` | A zero-sized value (e.g. closure with no captures) |

## Anatomy of a basic block

Each block = **statements** (mutate locals) + **one terminator** (decide where to go next).

```
bb0: {
    _5 = Arc::<T>::data(copy _1) -> [return: bb1, unwind continue];
    //  ^ statement-style assignment, but it's a CALL — calls are terminators
    //                          ^ on success → bb1
    //                                       ^ on panic → propagate up
}
```

Common terminators:

| Terminator | Meaning |
|---|---|
| `goto -> bbN` | Unconditional jump |
| `return` | Return `_0` to caller |
| `switchInt(_x) -> [1: bb2, otherwise: bb3]` | If/match branch |
| `call f(args) -> [return: bbN, unwind: bbM]` | Function call with success + panic paths |
| `drop(_x) -> bbN` | Run drop glue for `_x`, then continue |
| `resume` | Continue unwinding a panic |

## Worked example — `Arc::drop`

Source:
```rust
fn drop(&mut self) {
    if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
        fence(Ordering::Acquire);
        drop(unsafe { Box::from_raw(self.ptr.as_ptr()) })
    }
}
```

MIR shape:
```
bb0:  _5 = Arc::data(copy _1) -> bb1                  // self.data()
bb1:  _4 = &(*_5).0                                    // &ref_count
      _2 = AtomicUsize::fetch_sub(_4, 1, Release) -> bb2
bb2:  switchInt(_2) -> [1: bb3, otherwise: bb5]        // == 1?
bb3:  fence(Acquire) -> bb4
bb4:  _6 = Box::from_raw(...) -> bb5
bb5:  drop(_6) -> bb6                                  // ← T's destructor runs here
bb6:  return
```

The high-level `if` becomes `switchInt`. The implicit `Box` drop becomes an explicit `drop` terminator. **MIR makes invisible Rust semantics visible.**

## `copy _1` does NOT copy data

When you see `_5 = Arc::data(copy _1)`:

- `_1` is `&self` — a pointer (8 bytes).
- `copy _1` copies **the pointer**, not the `Arc`, not the heap data.
- References are `Copy`, so MIR uses `copy` instead of `move`.
- At the machine level both compile to the same load. The difference is a semantic marker for the borrow checker.

A real data copy looks like `Clone::clone(...)` as a call terminator.

## Stdlib functions are opaque calls

When you dump your crate's MIR, stdlib functions like `try_update`, `fetch_sub`, `Box::from_raw` appear as **single-line call terminators**. Their bodies are not inlined — that's what you want, otherwise the dump would be enormous.

To peek inside stdlib MIR you'd need to rebuild libstd with dump flags. Skip this for learning.

## What MIR is good for vs other tools

| Question | Tool |
|---|---|
| `copy` vs `move`? | **MIR** |
| Is `.clone()` actually called? | **MIR** (look for `Clone::clone`) |
| Where does drop run? | **MIR** (`drop` terminators) |
| Autoderef / autoref resolution? | `-Z unpretty=hir,typed` |
| What does `#[derive(Clone)]` generate? | `cargo expand` |
| Resolved method on `.foo()`? | rust-analyzer hover, or HIR |
| Redundant clones in my code? | `cargo clippy` |
| Final inlined output? | `--emit=llvm-ir` or `--emit=asm` |

## Practical workflow

```bash
# Tight loop for one function:
cargo +nightly rustc --lib -- -Z unpretty=mir 2>/dev/null | less
# /drop  ← search for the function in less

# Macro / derive expansion:
cargo install cargo-expand
cargo expand --lib basic_ref_counting

# Method resolution + autoref:
cargo +nightly rustc --lib -- -Z unpretty=hir,typed 2>/dev/null | less

# Quick experiments without a project:
# → godbolt.org (Compiler Explorer) — MIR + asm side by side
```

## How to read MIR in practice

1. Find the function — search for `fn name(` in the dump.
2. Read the **locals declarations** at the top to learn the type of each `_N`.
3. Start at `bb0`, follow terminators to trace control flow.
4. Pay attention to `drop` terminators — that's where destructors fire.
5. Ignore `(cleanup)` blocks on first read — they're panic-unwinding paths.
6. Watch `copy` vs `move` to understand ownership flow.

The "aha" moment usually comes from seeing how a single line of source explodes into several blocks — every call is a potential panic point, every scope-exit is a potential drop site, and MIR makes all of it explicit.
