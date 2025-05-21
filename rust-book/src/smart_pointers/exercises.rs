use std::{mem, ops::{Deref, DerefMut}};


pub fn my_list() {
    
    #[derive(Debug)]
    enum MyList {
        Cons(i32, Box<MyList>),
        Nil,
    }

    impl MyList {
        fn print(&self) {
            // This is how i iterate through `Enum` i just make recursion
            match self {
                MyList::Cons(value, next, ) => {
                    print!("{} -> ", value);
                    next.print();
                }
                MyList::Nil => {
                    println!("Nil");
                }
            }
        }

        fn sum(&self) -> i32 {
            match self {
                MyList::Cons(value, next ) => {value + next.sum()},
                MyList::Nil => 0
            }
        }
    }

    let list = MyList::Cons(1, Box::new(MyList::Cons(2, Box::new(MyList::Cons(3, Box::new(MyList::Nil))))));
    list.print(); // should print: 1 -> 2 -> 3 -> Nil
    println!("{}", list.sum()); // should print: 6
}

pub fn heal_allocation() {

    struct Data {
        buffer: [u8; 1000]
    };

    let stack_data = Data { buffer: [0; 1000]};
    let heap_data = Box::new(Data { buffer: [0; 1000]});

    println!("Stack size: {}", mem::size_of_val(&stack_data));
    println!("Boxed size: {}", mem::size_of_val(&heap_data));

}

pub fn ownership() {
    let boxed_string = Box::new(String::from("your text"));

    fn take_boxed_string(some_str: Box<String>) { // move the ownership of the box string into the function
        println!("Print the boxed string: {}", some_str);
    }

    take_boxed_string(boxed_string);
    // println!("Original value: {}", boxed_string); // Compiler-error here
}

// ========================== Deref trait

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn implement_deref_trait() {


    let x = 5;
    let my_box = MyBox(x);

    assert_eq!(5, *my_box);
}

pub fn deref_coercion_on_fn_call() {


    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }   

    let my_name = MyBox(String::from("Alice"));

    greet(&my_name);
}

pub fn deref_mut() {
    let mut my_box = MyBox(String::from("Hello"));
    my_box.push_str(", world!");
    assert_eq!(&*my_box, "Hello, world!");
}