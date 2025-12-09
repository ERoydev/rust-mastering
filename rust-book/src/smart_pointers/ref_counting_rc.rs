use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


// Rc stands for Reference Counted pointer.
// It enables multiple ownership of the same data.
// Multiple Rc instances can point to the same heap-allocated value.
// The data is read-only shared (immutable) — no thread-safety guarantee (not Send or Sync).

/* 
Why use Rc?
Imagine you have some data that multiple parts of your app want to read and share, but:
    - You don’t want to clone the data (which could be expensive).
    - You don’t want to transfer ownership.
    - You want Rust to track how many owners there are, and automatically free the data when nobody owns it anymore.
    - Rc does exactly that using reference counting
*/
use List::{Cons, Nil};

pub fn ref_counting_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // clone to pass a into b and c => doesn't make deep copy of the data, only increments the reference count
    // a.clone() would do the same since `a` is Rc initiliazed but current syntax is prefferr. ed
    let c = Cons(4, Rc::clone(&a));


    println!("Reference count of a = : {}", Rc::strong_count(&a));

}