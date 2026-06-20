use std::{pin::Pin, task::Context};

// Anathomy of the Future trait
pub trait FutureExample {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollExample<Self::Output>;
}

pub enum PollExample<T> {
    Ready(T),   // The future has completed with value T
    Pending,    // The future is not ready yet — call me back later
}

// Example of a simple future that is always ready
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

// ======= Exercise: Implement a CountdownFuture
pub struct CountdownFuture {
    n: i32,
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<&'static str> {
        if self.n == 0 {
            return Poll::Ready("Liftoff!");
        }

        self.n -= 1;


        Poll::Pending
    }
}

#[cfg(test)]
pub mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use crate::chap_2_future_trait::CountdownFuture;

    fn noop_raw_waker() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            noop_raw_waker()
        }
        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}

        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }

    fn noop_waker() -> Waker {
        // Safe: no-op waker never dereferences the data pointer.
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }

    #[test]
    pub fn test_future() {
        let mut countdown = CountdownFuture { n: 2 };
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        assert_eq!(Pin::new(&mut countdown).poll(&mut cx), Poll::Pending);
        assert_eq!(Pin::new(&mut countdown).poll(&mut cx), Poll::Pending);
        assert_eq!(
            Pin::new(&mut countdown).poll(&mut cx),
            Poll::Ready("Liftoff!")
        );


    }
}