use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static X: AtomicI32 = AtomicI32::new(0);

pub fn spawning_and_joining() {
    X.store(1, Relaxed); 
    let t = std::thread::spawn(f); // creates a happens-before relationship with all operations before the spawn
    // So here inside the thread, main may execute `store 2` before or after the spawned thread loads `X`, 
    // because there is no synchronization between the store of 2 and the load in f
    
    X.store(2, Relaxed);
    t.join().unwrap(); // creates another happens-before relationship with all operations in the spawned thread and any operations after .join
    X.store(3, Relaxed); 
}

fn f() {
    let x = X.load(Relaxed);
    println!("Loaded x: {x}");
    assert!(x == 1 || x == 2);
}