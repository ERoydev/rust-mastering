/*
Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

    - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    - Box<T> allows immutable or mutable borrows checked at compile time;
    - Rc<T> allows only immutable borrows checked at compile time; 
    - RefCell<T> allows immutable or mutable borrows checked at runtime.
    - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

// Interior mutability means = A value that is normally immutable from the outside can mutate its internal state safely using special types.
/*

Commont interior Mutability Types
    - RefCell<T> - Thread Safe: No - Works with: Rc<T> -> Allowing me to mutate data, even when the data declared is immutable !!!!
    - Mutex<T> - Thread Safe: Yes - Works with: Arc<T>
    - Cell<T> - Thread Safe: No - Works with: `Copy` types only
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // Wrap to apply interior mutability pattern
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), 
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // with .borrow_mut() i get mutable borrow on the inside while in the outside send is defined with immutable `self`
            let mut one_borrow = self.sent_messages.borrow_mut();

            // let mut two_borrow = self.sent_messages.borrow_mut(); // I cannot have two mutable borrows

            one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// ======================= With this approach i can allow multiple owners of mutable data 
/*
🔁 How does Rc<RefCell<T>> work?
    - Rc<T> gives you shared ownership (reference counting).
    - RefCell<T> gives you interior mutability (mutable borrow at runtime).
    - Together: You can share and mutate data across multiple owners — safely, as long as you're careful.

*/


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn multiple_owner_of_mutable_data() {
    // Shared, mutable vector
    let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&shared_vec);
    let b = Rc::clone(&shared_vec);

    a.borrow_mut().push(4);
    b.borrow_mut().push(5);

    println!("{:?}", shared_vec.borrow()); // [1, 2, 3, 4, 5]
}