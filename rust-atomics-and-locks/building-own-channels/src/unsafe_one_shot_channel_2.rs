/*
 So the idea is to build a Channel for communication that sends exactly one message from one thread to another.

 The idea is to use UnsafeCell for storage and AtomicBool to indicate its state (whether it is ready for consumption).

 Before the message is sent, the channel is "empty" and does not contain any message of type T yet.

 We could use Option<T> inside the cell to allow for the absence of a T.
 This will waste valuable memory, since our atomic boolean already tells us whether there is a message or not.

 For that reason, we will use std::mem::MaybeUninit<T>, which is the bare-bone version of Option<T>, and we will manually keep track if it has been initialized or not.
*/

use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    // Safety: Only call this once!
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Ordering::Relaxed) {
            panic!("can't send more the one message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release);
    }

    /// Panics if no message is available yer,
    /// or if the message was already consumed.
    ///
    /// Tip: use `is_ready` to check first.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no message available!");
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

pub fn unsafe_one_shot_channel() {
    use std::thread;
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark(); // sending thread will `unpark` the receiving thread, indicating that we have a message to receive
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}
