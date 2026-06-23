use std::future::{self, Future, poll_fn};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::pin::Pin;
use std::sync::atomic::Ordering;

/// The simplest possible executor: busy-loop poll until Ready, cp_2 in test i have same stuff like this
fn block_on<F: Future>(mut future: F) -> F::Output {
    // Pin the future on the stack
    // SAFETY: `future` is never moved after this point — we only
    // access it through the pinned reference until it completes.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // Create a no-op waker (just keeps polling — inefficient but simple)
    fn noop_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), vtable)
    }

    // SAFETY: noop_raw_waker() returns a valid RawWaker with a correct vtable.
    let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    // Busy-loop until the future completes
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => {
                // A real executor would park the thread here
                // and wait for waker.wake() — we just spin
                std::thread::yield_now();
            }
        }
    }
}


fn usage_of_block_on() {
    // Usage of block_on:
    let result = block_on(async {
        println!("Hello from our mini executor!");
        42
    });
    println!("Got: {result}");
}


// ======== EXERCISE: Spurious-Wake-Safe Flag Future

struct FlagFuture {
    flag: Arc<AtomicBool>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
}

impl FlagFuture {
    fn new() -> FlagFuture {
        FlagFuture { 
            flag: Arc::new(AtomicBool::new(false)), 
            waker_stored: Arc::new(Mutex::new(None)) 
        }
    }
}

async fn async_main() {
    // poll_fn and yield_now, utilities to avoid implementation of Future trait 
    
    // We know that implementing a `Future` means creating a struct and writing a `poll` method.
    // Using poll_fn i can skip implementing Future and writing a `poll` method
    // I just hand it a closure that is the poll logic
    let mut flag = AtomicBool::new(false);
    // Now the idea is tat instead of creating a stuct + Future, 
    // the state lives in captured variable like this `flag` instead of struct fields
    let fut = poll_fn(|cx| {
        if flag.load(Ordering::Relaxed) {
            println!("The future is ready!");
            Poll::Ready(())
        } else {
            flag.store(true, Ordering::Relaxed);
            cx.waker().wake_by_ref();
            println!("The future is still not ready!");
            Poll::Pending
        }
    });

    // logically i use it when i need a one-off future with local state and don't want to write whole struct + impl Future
    let result = fut.await;

}

pub fn run() {
    block_on(async_main());
}

// Conceptually this is what `yield_now()` does under the hood
async fn yeild_now_under_the_hood() {
    // It is used liek that
    // tokio::task::yield_now().await;
    // or in the futures crate:
    // futures::task::yield_now().await;

    // Conceptually, this is all yield_now() is:
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref(); // "wake me immediately after yielding"
            Poll::Pending
        }
    }).await;
}