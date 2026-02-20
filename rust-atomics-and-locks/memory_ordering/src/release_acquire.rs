use std::{fs, sync::atomic::{AtomicBool, AtomicU64, Ordering::{Acquire, Relaxed, Release}}, time::Duration};

static DATA: AtomicU64 = AtomicU64::new(0);

// So in simpler terms i just use Release and Acquire on this Atomic variable, to say that in order to Acquire the data it must first be Released
static READY: AtomicBool = AtomicBool::new(false);

pub fn release_acquire() {
    std::thread::spawn(|| {
        // So since the release flag is preventing other threads to use DATA before it is stored, i don't even need this DATA to be an atomic variable
        // I could use unsafe block to work with normal u64 variable
        DATA.store(123, Relaxed);
        READY.store(true, Release); // Everything from before this store is guaranteed to be happened before this `Release` ...
    });

    while !READY.load(Acquire) { // .. is visible after this loads `true`
        std::thread::sleep(Duration::from_millis(100));
        println!("waiting...")
    }
    println!("{}", DATA.load(Relaxed));
}

