use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
/*
// Set the value to `true`
// If the lock is already true, swap sets it to true again and returns true, so loop keeps spinning
// If the lock was false, swap sets it to true, returns false, so i exit the loop and acquire the lock.


.lock()
    - The loop tries to atomically change locked from `false` to `true`
    - If the lock is held the operation fails and returns Err (the loop continues)
    - On each failed attempt `spin_loop` is called, which tells the CPU the thread is `busy-waiting`, allowing for optimizations
    - When lock is free the operation succeeds and sets it to true and returns Ok. The loop exists and the lock is acquired.

*/

// Wrap the lock and provide access to the protected value. When guard is dropped i automatically unlock, and safely access the value only while holding the `Guard`.
#[derive(Debug)]
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// To make it behave like an (exclusive) ref. I need to implement the Traits for that
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

// Allowing completely remove the unsafe unlock method, so when Guard is dropped the value taken from the .lock() is dropped too
impl<T> Drop for Guard<'_, T> {
    // So when the Guard holding the locked value is dropped, it is automatically `unlocked`, so lock is released safely and automatically when goes out of scope.
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// Implement Sync only on generic T which already implements the Send Trait.
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // So without Guard i would need a way to give `a'` a lifetime to this until the next call to .unlock() on self, or when self is dropped
    // Without a guard, you cannot safely and automatically enforce that the lock is held only between lock() and unlock()—the borrow checker cannot track this pattern.
    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        // I can use the compare and exchange instead of swap here
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }
        Guard { lock: self } // Wrap the lock and provide access to the protected value. When guard is dropped i automatically unlock, and safely access the value only while holding the `Guard`.

        // while self.locked.swap(true, Ordering::Acquire) {
        //     std::hint::spin_loop(); // Tells the processor that the thread is in a spin-wait state, CPU can optimize the power usage or scheduling
        // }
    }

    // So this is handled by the Guard
    #[allow(dead_code)]
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub fn spin_lock() {
    let mut data = Box::new([0u64; 10]);
    let spin_lock = SpinLock::new(&mut *data);

    std::thread::scope(|s| {
        s.spawn(|| {
            let mut rec_data = spin_lock.lock();
            rec_data[0] = 12;
        }); // So because of Guard when this gets dropped it is unlocked()
    });

    std::thread::scope(|s| {
        s.spawn(|| {
            let mut rec_data = spin_lock.lock();
            rec_data[0] = 24;
        });
    });

    println!("Data: {:?}", data);
}
