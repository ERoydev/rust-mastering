use std::cell::RefCell;
use std::rc::{Rc, Weak};


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Weak<T> is the version of `Rc` holding a non-owning ref to the managed allocation
//      - that allocation is accessed by calling `.upgrade()` on the Weak pointer, returns `Option<Rc<T>>`

pub fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // holding non owning reference
        children: RefCell::new(vec![]),
    });

    
    println!("leaf parent = {:?} ", leaf.parent.borrow().upgrade());
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("leaf parent = {:?} ", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
}