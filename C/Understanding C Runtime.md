# Understanding C Runtime

## Intro 
Tipically when i compile a Rust program with `std`
```
rustc your_program.rs                                                                                                                                            
    ↓                                                                                                                                                        
links against libc (glibc or musl)                                                                                                                             
    ↓                                                                                                                                                        
libc provides crt0/crt1 which provides _start                                                                                                                
    ↓                                                                                                                                                        
_start → __libc_start_main → main()   
```
The toolchain does that automatically - `rustc` tells the linker to include `crt1.o`, link `libc.so/libc.a` and everything just works.

So basically even if i have this code 
```rust
fn main() {                                                                                                                                                    
    let v = vec![1, 2, 3];
    println!("{:?}", v);                                                                                                                                         
}
```

Under the hood this happens:
```
_start          (from crt1.o, you never see it)                                                                                                                  
    → __libc_start_main                                                                                                                                            
        → set up malloc arena    ← so Vec can allocate
        → set up stdio           ← so println! can write to stdout                                                                                                 
        → set up TLS             ← so thread-locals and errno work                                                                                                 
        → main()                 ← now your code runs  
```

`Vec::push` calls Rust's `alloc::alloc`, which delegates to libc's malloc via the default allocator backend (__rdl_alloc), which ultimately calls the mmap syscall to get memory from the kernel. All of that plumbing was initialized by `__libc_start_main` before your code got control.    

## What is the C Runtime?

Before your `main()` function is executed, a few pieces of code run both before and after `main()`, preparing the environment for your program. These are called C runtime (CRT) startup objects:

* `crt0.o`, `crt1.o`, `crti.o`, and `crtn.o`

These are collections of startup routines, initialization code, standard library support, and system call wrappers that form the environment in which a C program executes. This code lives outside your application source, but is automatically linked in by the compiler driver (e.g., `gcc` or `clang`).

The compiler driver and linker implicitly include these startup object files and libraries. They contain assembly-level entry points and routines that:

- Initialize registers and the stack
- Set up program arguments (`argc`, `argv`, `envp`)
- Invoke global constructors (in C++ programs)
- Call your `main()` function
- Handle the return from `main()` and pass the exit status to the operating system

## What `crt0.o` (or `crt1.o` does in Modern Toolchains)
The `crt0.o` (C runtime zero) is a small object file containing the actual entry point routine, often name `_start` that is responsible to include:

1. Program initialization
2. Transferring Control to `main()`
3. Cleaning Up

Generally they contain:
- Low-level assembly code repsonsible for setting up the runtime.
- A symbol name `_start`/`__start` that acts as the entry point.
- A call to `main()`

## Linking Phase
Tipically when i link the program, the linker automatically pulls in `crt0.o` (or `crt1.o`) from the C library implementation (e.g., `glibc` or `musl`) or from the compiler toolchain. This happens behind the scenes unless i explicitly disable it (compiler flags like `nostartfiles`).


# Resources
- [1] -> https://inferara.com/blog/c-runtime/
- [2] -> https://etherealwake.com/2021/09/crt-startup/
- [3] -> https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/7_GNUPro_Embedded_Development/embcrt0_the_main_startup_file.html
