use std::cell::RefCell;
use std::os::unix::raw::time_t;
use std::rc::{Rc, Weak};

/*
Definition:
    A Weak<T> is a non-owning pointer into an Rc/Arc allocation
        that may no longer contain a live value, and must be upgraded to access it safely.

Strong Vs Weak:
    - Strong ref -> (Rc<T> or Arc<T>), Kepp the value alive. The data cannot be dropped while strong ref exist.
    - Weak ref -> Don't keep the value alive. The data can be dropped even if weak ref still exist.
*/

// The problem here is if Parent and Child hold a Strong reference to each other, neither can ever be dropped from memory
// The ref counter never reaches zero because they're keeping each other alive - like two people holding onto each other in quicksand.
// So called `Reference Cycle` it's a memory leak

// Resource -> https://dev.to/masteringbackend/what-is-a-weak-pointer-in-rust-and-why-it-matters-29m5

#[derive(Debug)]
struct Parent {
    child: Option<Rc<RefCell<Child>>>, // strong ref
}

#[derive(Debug)]
struct Child {
    parent: Option<Rc<RefCell<Parent>>>, // strong ref
}

// A Doubly-Linked List: Weak pointers in Action

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<NodeRef>,     // if next then next is child
    prev: Option<WeakNodeRef>, // if prev then prev is parent
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: Option<NodeRef>,
    tail: Option<WeakNodeRef>, //
}

enum TraverseOrder {
    Forward,
    Backward,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        // This match check if the DoublyLinkedList is empty at all. No one have set tail or head. Because if no tail set then no head exists as well
        match &mut self.tail {
            Some(t) => {
                // This checks if the tail weak pointer actually holds something and convert to strong pointer
                if let Some(tail_strong_ptr) = t.upgrade() {
                    let mut tail_node = tail_strong_ptr.borrow_mut();

                    // Set the parent node of the new_node
                    new_node.borrow_mut().prev = Some(t.clone()); // just bump the ref_counter `not real clone`
                    // Set the child node of the old tail
                    tail_node.next = Some(new_node.clone()); // Bump the ref counter
                    // Set a new tail to this list
                    self.tail = Some(Rc::downgrade(&new_node));
                }
            }
            // if empty
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
                return;
            }
        }
    }

    fn print_forward(&self) {
        let mut data_for_print: Vec<String> = vec![];
        let head_node = &self.head;

        Self::traverse_list(head_node, &mut data_for_print, TraverseOrder::Forward);
        println!("{:?}", data_for_print);
    }

    fn print_backward(&self) {
        let mut data_for_print: Vec<String> = vec![];
        // let tail_node = &self.tail;

        let node_ref = match &self.tail {
            Some(weak_ref) => {
                let tail_node= weak_ref.upgrade();
                tail_node
            },
            None => {
                None
            }
        };

        Self::traverse_list(&node_ref, &mut data_for_print, TraverseOrder::Backward);
        println!("{:?}", data_for_print);
    }

    fn traverse_list(
        parent_node: &Option<NodeRef>,
        trace_vec: &mut Vec<String>,
        order: TraverseOrder,
    ) {
        match parent_node {
            Some(node) => {
                let inner_node = node.borrow();
                trace_vec.push(inner_node.data.to_string());

                // Recursively pass through all the childs/parents in order for each node
                match order {
                    TraverseOrder::Forward => {
                        let child_node = &inner_node.next;
                        Self::traverse_list(child_node, trace_vec, order);
                    }
                    TraverseOrder::Backward => {
                        if let Some(prev_node) = &inner_node.prev {
                            let parent_node: Option<NodeRef> = prev_node.upgrade();
                            Self::traverse_list(&parent_node, trace_vec, order);
                        }
                    }
                }
            }
            None => {
                trace_vec.push("None".to_string());
            }
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("Forward:");
    list.print_forward(); // 1 -> 2 -> 3 -> None

    println!("Backward:");
    list.print_backward(); // 3 -> 2 -> 1 -> None
}
