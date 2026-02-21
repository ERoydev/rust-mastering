
use std::sync::atomic::fence;
use std::thread;
use std::time;
use std::sync::atomic::{AtomicU16, Ordering::{Relaxed, Acquire, Release}};

static A: AtomicU16 = AtomicU16::new(0);

/*
So as you can see i have used fence to separate the Memory ordering from the atomic operation

So now the happen-before relation ship is between the release-fence and acquire-fence and everything between can be what i want,
    i could have if-else control flows or anything else 
*/

pub fn simple_fence_example() {
    A.store(1, Relaxed);
    let _t = thread::spawn(|| {
        // Old Code
        // let x = A.load(Acquire);

        // Using fence
        let x = A.load(Relaxed);
        fence(Acquire); // Tipicaly fence is not tied to one atomic operation i could use it for multiple atomic operations if i want

        println!("{}", x == 1);
        println!("{}", x == 2);
        println!("{}", x == 3);

    });
    A.store(2, Relaxed);

    // Old code
    // A.store(3, Release);

    // New approach using fences
    fence(Release);
    A.store(3, Relaxed);
    thread::sleep(time::Duration::from_millis(10));
}
