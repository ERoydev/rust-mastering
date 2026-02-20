use std::{fs, sync::atomic::{AtomicBool, AtomicU64, Ordering::{Acquire, Relaxed, Release}}, time::Duration};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: We hold the exclusive lock so nothing else is accessing DATA.
        unsafe {
            let ptr = std::ptr::addr_of_mut!(DATA);
            (*ptr).push('!');
            println!("Data: {:?}", *ptr);
        }

        LOCKED.store(false, Release);
    }
    /*
    So the idea is they 
    
        any previous access to DATA(the unsafe block) happened-before the release-store of false to LOCKED,
        which in turn happened-before the next acquire-compare-and-exchange operation that changed that `false` to `true`
        which happened-before the next access to DATA.
     */

    // This is the implementation without using `compare_exchange`
    // while LOCKED.load(Acquire) {}
    // LOCKED.store(true, Relaxed);
    // // Safety: We hold the exclusive lock so nothing else is accessing DATA.
    // unsafe {
    //     let ptr = std::ptr::addr_of_mut!(DATA);
    //     (*ptr).push('!');
    // }
    // LOCKED.store(false, Release);
}

pub fn example_locking() {
    std::thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}

