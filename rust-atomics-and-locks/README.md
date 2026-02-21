# Rust Atomics and Locks Book by Mara Bos

This repository contains my code written while studying the "Rust Atomics and Locks" [BOOK](https://mara.nl/atomics/). For each chapter, I include code snippets along with explanatory comments to illustrate key concepts and patterns.


<p align="center">
  <img src="https://mara.nl/atomics/cover.jpg" width="300" alt="Rust Atomics and Locks Cover">
</p>

---

## Chapters

* **Chapter 1: Basics of Rust Concurrency**
    Introduces threads, `Send/Sync traits`, `Arc`, `Mutex`, `RwLock`, and basic interior mutability `(Cell/RefCell)`.
* **Chapter 2: Atomics**
    Explores atomic `load/store`, `fetch-and-modify`, and `compare-and-exchange` operations for lock-free data manipulation.
* **Chapter 3: Memory Ordering**
    Covers the Rust memory model, reordering, happens-before relationships, `relaxed`, `release/acquire`, and sequentially consistent ordering.
* **Chapter 4: Building Our Own Spin Lock**
    Demonstrates how to create a simple, low-level spin lock using UnsafeCell and atomic operations.
* **Chapter 5: Building Our Own Channels**
    Covers creating synchronization channels for inter-thread communication.
* **Chapter 6: Building Our Own "Arc"**
    Explores implementing a custom Arc<T> (Atomic Reference Counter).
* **Chapter 7: Understanding the Processor**
    Explains how atomic operations map to hardware, focusing on CPU architectures like Intel and ARM.
* **Chapter 8: Operating System Primitives**
    Covers using OS-level features like parking and futexes to build efficient, non-spinning locks.
* **Chapter 9: Building Our Own Locks**
    Combines previous chapters to create fully functional mutexes and condition variables.
* **Chapter 10: Ideas and Inspiration**
    Discusses more advanced patterns, such as lock-free linked lists and RCU (Read-Copy-Update) patterns.


Usefull Links:

For Memory Ordering this explains it good -> [here](https://www.youtube.com/watch?v=373srjM3Sbw) at 16:00 min