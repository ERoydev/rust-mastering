## 1. Understand hidden costs

### MIR/LLVM IR helps you see:

- extra clones you didn’t notice
hidden allocations (Vec, String, format!)
temporary values being created
drop calls happening implicitly

## 2. Understand ownership cost (Rust-specific)

### MIR shows:
  - when values are moved
  - when values are dropped early
  - when borrows end

This helps you fix:
  - unnecessary cloning
  - unnecessary scopes
  - accidental re-borrowing

## 3. Optimize performance behavior

### LLVM IR helps you see:

  - inlining decisions
  - loop optimizations
  - constant folding
  - removal of dead code

So you can answer:
  - “Why is this slower than I expect?”

## Mental Model
```
Rust code
  ↓
HIR = cleaned syntax
  ↓
MIR = ownership + logic graph
  ↓
LLVM IR = optimized low-level code
  ↓
CPU execution (not visible here)
```


## 🧠 Rust Optimization Debug Flow 

If you want to understand what your Rust code is really doing:

### If you want to see how Rust rewrites your code →

```bash
cargo rustc -- -Z unpretty=hir-tree
```

### If you want to see ownership, moves, and control flow →

```bash
cargo rustc -- --emit=mir
```

### If you want to see low-level compiler output →

```bash
cargo rustc -- --emit=llvm-ir
```

### If you want to find real performance issues →

```bash
cargo bench
cargo flamegraph
```
