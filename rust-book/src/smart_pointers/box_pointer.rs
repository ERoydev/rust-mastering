
/*
Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. 
But they don’t have many extra capabilities either. You’ll use them most often in these situations:
    - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    - When i use recursive types, rust must know size of all types at compile time, so with Box i can stoore that recursive type on the heap

🧠 Mental Model:
Think of it like this:
    1. Box<T> = stack pointer ➡️ heap value.
    2. Dropping the box ➡️ automatically frees heap value.
    3. Moving the box ➡️ transfers the heap ownership.

*/

// The idea is that Box<T> stores the List on the heap and i just have a pointer to that heap allocation in compile time
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>), // -> (i32, pointer to List on the heap) => So now pointer is a known size since its just a pointer
    Nil,
}

use List::{Cons, Nil};

pub fn box_pointer() {
    // 1. Storing data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // 2. Transferring ownership
    let b = Box::new(String::from("Hello"));
    let c = b;
    // println!("b = {}", b); // This will cause a compile-time error because ownership has been transferred
    fn take_boxed_string(c: Box<String>) { // move the ownership of the box string into the function
        println!("Print the boxed string: {}", c);
    }
    take_boxed_string(c);
    // println!("c: {}", c); // compile-time errro because ownership is transfered and dropped in the function

    // 3. Trait objects
    let s: &str = "Hello, world!";
    let t: Box<dyn std::fmt::Display> = Box::new(s);
    println!("{}", t);

    // 4. Recursive types
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);
}