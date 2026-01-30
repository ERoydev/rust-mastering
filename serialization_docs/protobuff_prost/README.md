# Protocol Buffers in Rust with Prost

Educational project demonstrating how to use Protocol Buffers in Rust using the `prost` crate.

## How It Works

1. **Define your schema** in `.proto` files (`src/employees/employees.proto`)
2. **Build script** (`build.rs`) automatically compiles `.proto` to Rust during build
3. **Use generated types** in your code by including them from `OUT_DIR`

## Key Files

- `src/employees/employees.proto` - Protocol Buffers schema definition
- `build.rs` - Compiles proto files using `prost-build`
- `src/main.rs` - Shows how to use the generated Rust types
- `Cargo.toml` - Dependencies: `prost` (runtime) + `prost-build` (build-time)

## Run

```bash
cargo run
```

The proto compilation happens automatically during build. Generated code lives in `target/debug/build/protobuff_crash_course-*/out/employees.rs`.

## No Manual Compilation Needed

Unlike other languages, you don't need to manually run `protoc`. The `prost-build` crate handles everything during `cargo build`.
