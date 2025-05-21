
// - With Rc<T>, RefCell<T> smart pointers => I can create references where items refer to each other in cycle.
// This creates a memory leaks, because the references count of each item in the cycle will never reach 0, and the values will never be dropped.
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
// https://www.youtube.com/watch?v=pIVZRDFAUyc&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=27 => Explained why it happens heap memory leak
// Because when i make cycle even if the stack pointer to the heap is dropped -> the heap allocated data stays there because they are referencing each other.

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn reference_cycles() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

