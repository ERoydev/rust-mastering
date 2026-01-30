fn main() {
    // I can just use `prost` to generate the Rust code for the .proto file
    prost_build::compile_protos(&["src/employees/employees.proto"], &["src/"]).unwrap();
}

// it acts as a build-time code generator For Protocol Buffers.
// Reads my .proto files and generates Rust code automatically during the build process
// The generated Rust code is placed in a special build output directory (OUT_DIR), and i include it in my project with `include!` macro.

// The OUT_DIR environment variable is automatically set by Cargo for build scripts like build.rs.