use std::{pin::Pin, task::Context};

// ======== ANATHOMY OF THE FUTURE TRAIT
pub trait FutureExample {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollExample<Self::Output>;
}

pub enum PollExample<T> {
    Ready(T),   // The future has completed with value T
    Pending,    // The future is not ready yet — call me back later
}

// ======== EXAMPLE - of a simple future that is always ready
use std::future::Future;
use std::task::{Poll};

// A future that returns 42 immediately
struct Ready42;

impl Future for Ready42 {
    type Output = i32; // What the future eventually produces

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<i32> {
        Poll::Ready(42) // Always ready — no waiting
    }
}

// ======= EXERCISE: Implement a CountdownFuture
pub struct CountdownFuture {
    n: i32,
}

impl CountdownFuture {
    fn new(n: i32) -> Self {
        CountdownFuture { n }
    }
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if self.n == 0 {
            return Poll::Ready("Liftoff!");
        }

        self.n -= 1;
        println!("n: {}", self.n);
        cx.waker().wake_by_ref(); // tells the excutor "this future is ready to be polled again.". Calls wake without consuming the waker `.wake()` consumes it

        Poll::Pending // Gives up the thread right now 
    }
}

#[cfg(test)]
pub mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use crate::cp_2_future_trait::CountdownFuture;

    /*
    Generally to implement a Waker we use `RawWaker` + `vtable` machinery and this is liek a contract between my future and the executor used.
    The executor defines what wake() actually does - because only the executor knows how it soters and schedules futures.

    That why `RawWaker` uses raw pointers and a vtable - so any executor can plut in its own scheduling logic.

    For that reason we use `noop` Waker, because we just execute the futures in loop and just call poll and it works, no reschedule, no notifications.

    The vtable needs 4 functions like `clone, wake, wake_by_ref, drop`
    */

    static VTABLE: RawWakerVTable = create_vtable();

    // In real executor does do stuff, but here are just empty
    fn wake(_: *const ()) {
        // In real executor it would
        // - Use the data pointer to find the future's queue slot
        // - Push it back onto the queue
        // - Drop the data
    }
    fn wake_by_ref(_: *const()) {}
    fn drop(_: *const ()) {}

    fn clone(_: *const()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    const fn create_vtable() -> RawWakerVTable {
        RawWakerVTable::new(clone, wake, wake_by_ref, drop)
    }

    // Raw waker is the unsafe low-level building block. Waker is the safer wrapper around it and the rest of async ecosystem works with.
    // So the chain is RawWaker -> Waker -> Context -> poll()
    fn noop_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    // The actuall wrapper that is used externally
    fn noop_waker() -> Waker {
        unsafe { Waker::from_raw(noop_raw_waker())}
    }



    #[test]
    pub fn test_future() {

        let mut countdown = CountdownFuture::new(2);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // This is like a mocked executor idea: In reality the executor will receive `waker.wake()` 
        // and then the future will be scheduled in the queue again
        // and then polled again like that

        assert_eq!(Pin::new(&mut countdown).poll(&mut cx), Poll::Pending);
        assert_eq!(Pin::new(&mut countdown).poll(&mut cx), Poll::Pending);

        assert_eq!(
            Pin::new(&mut countdown).poll(&mut cx),
            Poll::Ready("Liftoff!")
        );
    }
}