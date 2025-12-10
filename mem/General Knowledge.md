# How Rust Binaries Work

Resource used: https://www.youtube.com/watch?v=7_o-YRxf_cc&t=904s

### So in rust when i run either with cargo build or directly rust creates an executable file

## 1. Rust produces a native executable
Each OS uses different executable format 

- **Linux:** ELF64
- **macOS:** Mach-O
- **Windows:** PE

> These formats only define how the binary is stored on disk.

---

## 2. Executable formats define structure
They specify things like:
- Where the header is
- Where the code is
- Where the static data is
- How the kernel should load it

> The execution concept is the same on all OSes, only the executable binary format my be different.

---

## 3. When you run the program, the kernel creates a process
- When the binary is executed, the kernel provides a continuous range of memory addresses for the program to use
- Gives the program its own virtual address space (fake continuous memory)
- The program becomes a `Process`

---

## 4. The kernel maps segments of the executable into memory
**Typical segments:**
- **text:** contains machine code instructions for the CPU (read-only, executable), also called `Code Segment`
- **data:** initialized global/static variables
- **bss:** uninitialized global/static variables (starts as zeroed memory)

**Extra:**
- OS maps arguments, environment variables, and some metadata
- Kernel also sets up:
  - The stack
  - Space for the heap

---

## 5. Stack
- A Process (running program) will have at least one thread of execution and each thread have its own `stack`.
- Each thread has its own stack
- The main thread usually gets 8 MB on Linux (not preallocated — only committed when used)
- Stacks grow downward (toward lower memory addresses)
- Rust lets you configure thread stack size

- Stack is managed only through `stack frame pointer`

If a function returns a reference to data on its own stack frame, that reference becomes invalid once the stack frame is overwritten (for example, when the function returns and another function call reuses that stack space).

---

## 6. Heap
- All threads share the same heap
- Heap grows upward (toward higher addresses)
- If a program allocates more heap memory than is currently available, the standard library (such as libc in C, or Rust's allocator) will request additional memory from the operating system, typically via a system call (like brk, sbrk)
- 
- Overall heap allocation is slower because it involves system calls and memory allocations to find free slot on the heap for every allocation

## 7. CPU Word Size (e.g., 64-bit)

A 64-bit CPU means:

- CPU registers are 64 bits wide
- Pointers are typically 64 bits
- The CPU can address a large virtual space (up to $2^{64}$ possible addresses)
- The ABI and data alignment rules follow 64-bit conventions 
