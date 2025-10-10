# Using CodeLLDB

1. First of all, `rust-analyzer` will automatically set itself in its **Settings** → **Debug** → **Debug: Engine** as:

   ```
   vadimcn.vscode-lldb
   ```

   This is the **CodeLLDB** adapter that `rust-analyzer` uses.

   - The `rust-analyzer` will create **Run Tests** and **Debug** buttons above your Rust test cases.
   - Using that debug engine, it will run the debugger in **VS Code**.
   - You can start debugging your test, and from the **DEBUG CONSOLE**, you can set a breakpoint (using LLDB syntax) in any file where you want to stop during debugging:

     ```sh
     breakpoint set --file /Users/emil/Programming/sbf/src/vm.rs --line 364
     ```

   - Then click **Continue** to go to that file and inspect the execution.

---

# Formatting

When debugging, you can format your error messages like this:

```rs
assert!(result.is_ok(), "Transaction failed: {:#?}", result.err());
```

- The `:#?` specifier will pretty-print the message with indentation and structure, making your debug output easier to read.
