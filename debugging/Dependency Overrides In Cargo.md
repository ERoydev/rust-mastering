# Dependency Overrides in Cargo (`patch.crates-io`)

When overriding dependencies using `[patch.crates-io]` to point to your own git forks, you must ensure that **all occurrences** of the dependency in the `cargo tree` are sourced from your fork. If any instance is not, the override will fail.

## Example

- **Forked dependency:**
  - `solana-bpf-loader-program v3.0.6 (https://github.com/ERoydev/agave.git?branch=ver-3.0.6#0bf8b0dc)`  
    _This comes from my fork._
- **Default dependency:**
  - `solana-account v3.1.0 (*)`  
    _This is the default appearance; you need to trace the graph to ensure it is overridden._

**Dependency graph illustration in my case:**

```
surfpool-cli → surfpool-core → litesvm → solana-bpf-loader-program
```

---

## My Process

1. **Apply patches incrementally:**
   - After each patch, inspect the output of `cargo tree` to verify that all occurrences (e.g., `litesvm`) are coming from your fork.
2. **Version alignment:**
   - My code expects `bpf-loader-program` version `3.0.6`, so I created a fork with that exact version.
   - Ensure your forked dependency matches the required version.
3. **Full override:**
   - By aligning versions and patching, I successfully forced all usages of the dependency (even those deep in the tree) to use my fork.

4. **Fork and create a branch using the remote tag for specific version**
   - Logically when i fork a repo i create a remote to the original repo and by using tag i create a branch with specific version

---

**Summary:**

- Use `[patch.crates-io]` to override dependencies with your fork.
- Check `cargo tree` after each change to confirm all instances are overridden.
- Ensure your forked dependency matches the required version to avoid conflicts.
