use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    // the compiler and CPU may reorder these operations within the same thread if it determines that the reordering does not affect the behavior of that THREAD.
    // That means that other threads my observe the effect of this operations in an order different from how they appear in my source code.
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);
    println!("{x} {y}");
}
// Explanation:
// Here if i run a and b concurrently in separate threads, the results can vary because i have no synchronization or memory ordering between them
// In other words Y.load have no happens-before relationship with Y.store, so it can read the value Y before it is stored by a.

// So with relaxed ordering, there is no guarantee that a's store to Y happens before b's load of Y.
pub fn happens_before_relationship() {
    a();
    b();
}
