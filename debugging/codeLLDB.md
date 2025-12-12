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

# Debugging the Fork with LLDB

## 1. Build tests without running them

This compiles the tests and produces an executable binary without executing it:


In this example i compile a specific `crate` that i want to debug from a workspace

```bash
cargo test -p surfpool-core --no-run
```

## 2. Launch LLDB using the generated test binary

Find the produced test executable under `target/debug/deps/` and start LLDB:

```bash
lldb target/debug/deps/surfpool_core-<some_hash>
```

## 3. Set a breakpoint on a specific file and line

Use the absolute file path and the line number you want to break at:

```bash
breakpoint set --file /Users/emilemilovroydev/Rust/projects/Solana/gimlet-debugger/surfpool-fork/crates/core/src/surfnet/svm.rs --line 500
```

## 4. Run the test binary inside LLDB

Start execution and LLDB will stop at your breakpoint:

```bash
run
```

## 5. If that work you can use CodeLLDB inside `.vscode/launch.json`.
The `launch.json` file will look something like this

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug surfpool_core binary (test build)",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/deps/surfpool_core-233e4c2f48bd8d26",
            "args": [],
            "cwd": "${workspaceFolder}",
            "stopOnEntry": false,
            "preRunCommands": [
                "breakpoint set --file ${workspaceFolder}/crates/core/src/surfnet/svm.rs --line 500"
            ]
        }
    ]
}
```

---

# Formatting

When debugging, you can format your error messages like this:

```rs
assert!(result.is_ok(), "Transaction failed: {:#?}", result.err());
```

- The `:#?` specifier will pretty-print the message with indentation and structure, making your debug output easier to read.
