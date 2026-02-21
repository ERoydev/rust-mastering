use std::sync::atomic::{AtomicPtr};
use std::sync::atomic::Ordering::{Release, Acquire};

#[derive(Debug)]
struct Data {
    name: String,
    age: u32
}

impl Data {
    pub fn new() -> Self {
        Self {
            name: String::from("Alice"),
            age: 24,
        }
    }
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut::<Data>()); // Set initial state of null pointer

    // If no `Release` store before `Acquire` load, the Acquire load will still read the current value of the atomic variable (act like Relaxed)
    let mut p: *mut Data = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(Data::new())); // Box allows me to get a raw-pointer of that data stored on the heap and creates something like memory leak
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut::<Data>(), p, Release, Acquire) {
            // `Acquire ` on failure synchronizes with the Release from the other thread that initialized this data first and commited `Release` on it.
            // So since this thread loses the race, compare_exchange returns the Err which is pointer(initialized data) from the other thread,
            // Thats why i need to drop my allocated memory for pointer B and use the pointer A

            // Safety: p comes from `Box::into_raw` right above, that wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p)});
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value. So here we make `order` assumption that the initializing will happen before this
    unsafe { &*p }

}

pub fn lazy_initialization_with_indirection() {
    // use an atomic pointer and use a null pointer for the initial state, 
    // use compare and exchange to atomically replace it with a pointer to a newly
    // allocated, fully initialized T which then can be read by other threads.

    let data = get_data();
    println!("Data received 1: {:?}", data);

    let data2 = get_data();
    println!("Data received 2: {:?}", data2);

}