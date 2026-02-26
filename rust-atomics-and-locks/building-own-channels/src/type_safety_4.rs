use std::{cell::UnsafeCell, mem::MaybeUninit, sync::{Arc, atomic::{AtomicBool, Ordering}}};

pub struct Sender<T> {
    pub channel: Arc<Channel<T>>,
}
pub struct Receiver<T> {
    pub channel: Arc<Channel<T>>,
}


pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });

    (Sender { channel: a.clone()}, Receiver { channel: a} )
}

struct Channel<T> {
    // Using MaybeUninit instead of Options to not waste memory resources
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send { }

// Implementations ----

impl<T> Sender<T> {
    pub fn send(self, message: T) {

    }
}


impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed)
    }

    // By using `self` this consumes the Receiver instance, ensuring that receive can only be called once for that instance
    // enforces the compile time that the channel can only be received from once, providing type safety
    pub fn receive(self) -> T {
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no message available!");
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}