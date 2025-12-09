use std::{cell::{Cell, Ref, RefCell}, rc::Rc};

// ========= This is the problematic code i have to find a solution for you can uncomment to inspect the errors ===============
// #[derive(Debug, Clone)]
// struct Node<'a> {
//     val: <i32>,
//     adjacent: Vec<&'a Node<'a>>
// }

// fn add_one(node: &Node) {
//     let adjacent_nodes = &node.adjacent;
//     for adj in node.adjacent {
//         add_one(&adj);   
//     }
// }

// // Graph = A -> B -> C
// pub fn main() {
//     let a = Node {
//         val: 1,
//         adjacent: vec![],
//     };

//     let b = Node {
//         val: 2,
//         adjacent: vec![&a],
//     };
//     let c = Node {
//         val: 3,
//         adjacent: vec![&b, &a],
//     };

//     add_one(&b);

//     dbg!(&a);
//     dbg!(&b);
//     dbg!(&c);
// }

// ================= Solution ===========

/*
Cell:
- Allows Mutation
- Grab a copy of what's inside

Note: It never gives out shared references (no dangling references or data consistency)
*/

#[derive(Debug, Clone)]
struct Node<'a> {
    val: Cell<i32>, // Cell allows us to mutate the contained value even if we only have a shared reference to that Cell
    // The way it does this is by allowing me to mutate the value, but never allowing me to get a shared reference
    // I can either `mutate the value` or i can `get a Copy` of the value inside
    // Cell has a .get() function to get a Copy of what is inside ( the thing i am putting in the Cell has to implement the Copy trait)
    adjacent: Vec<&'a Node<'a>>
}

fn add_one(node: &Node) {
    let curr_val = node.val.get(); // Copy of the i32, NOT A SHARED REFERENCE
    node.val.set(curr_val + 1); // 

    for adj in node.adjacent.iter() {
        /*  
        Notes from the video i need to check out, learn and understand:

            node.adjacent is moved due to this implicit call to .into_iter() -> The Error

            Explanation:
            By default in a for Loop if you do for each on some collection it's gonna implicitly call .into_iter(), which is gonna take ownership
            of the things that you are iterating over, so thats why i need to explicitly call `.iter()` which is going to grab a shared reference instead
        */
        add_one(&adj);   
    }
}
// Graph = A -> B -> C
pub fn main() {
    let a = Node {
        val: Cell::new(1),
        adjacent: vec![],
    };

    let b = Node {
        val: Cell::new(2),
        adjacent: vec![&a],
    };
    let c = Node {
        val: Cell::new(3),
        adjacent: vec![&b, &a],
    };

    add_one(&b);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

// So if i have a data that is copyable and you don't mind copying it the `Cell` might be the best mechanism for Interior Mutability



// ===================== RefCell example ====================
use std::{cell::{Cell, Ref, RefCell}, rc::Rc, thread};

#[derive(Debug, Clone)]
struct Node<'a> {
    val: RefCell<String>, // String does not implement Copy trait, that's why i use RefCell
    // So Cell will not work because the value is not Copyable or i dont want it to be copyable, because of performance reasons
    // RefCell can panic! if program attempts to get a mutable reference at the same time there's a shared reference out or vice versa
    // Cell is tipicaly more convenient way
    adjacent: Vec<&'a Node<'a>>
}

fn add_urgency(node: &Node) {
    let mut curr_val = node.val.borrow_mut(); // mutable ref to the actual value inside RefCell
    curr_val.push('!'); // modifies the original value, not a copy

    for adj in node.adjacent.iter() {
        add_urgency(&adj);   
    }
}
// Graph = A -> B -> C
pub fn main() {
    let a = Node {
        val: RefCell::new(String::from("aaa")),
        adjacent: vec![],
    };

    let b = Node {
        val: RefCell::new(String::from("bbb")),
        adjacent: vec![&a],
    };
    let c = Node {
        val: RefCell::new(String::from("ccc")),
        adjacent: vec![&b, &a],
    };

    // add_one(&b);
    add_urgency(&b);
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

// ======================== RwLock example ====================

// Mutex only one thread have access at a time.
// RwLock multiple threads can read data but only one thread can write data at a time.


#[derive(Debug, Clone)]
struct Node {
    val: RwLock<String>, // Similar to RefCell, but can be used for multithreaded situations
    // One thread can modify the data at any give time (Mutex logic)
    adjacent: Vec<Arc<Node>> // Now since i pass things to other threads i need to use Arc
}

fn add_urgency(node: &Node) {
    {
        let mut curr_val = node.val.write().unwrap(); 
        curr_val.push('!');
    } // release the lock

    for adj in node.adjacent.iter() {
        add_urgency(&adj);   
    }
}

// Graph = A -> B -> C
pub fn main() {
    let a = Arc::new(Node {
        val: RwLock::new(String::from("aaa")),
        adjacent: vec![],
    });

    let b = Arc::new(Node {
        val: RwLock::new(String::from("bbb")),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(Node {
        val: RwLock::new(String::from("ccc")),
        adjacent: vec![b.clone(), a.clone()],
    });

    // add_one(&b);
    add_urgency(&b);

    let t1_b = b.clone();
    let t1= thread::spawn(move || {
        add_urgency(&t1_b);
    });

    let t2_c = c.clone();
    let t2 = thread::spawn(move || {
        add_urgency(&t2_c);
    });

    t1.join();
    t2.join();
    dbg!(&*a);
    dbg!(&*b);
}