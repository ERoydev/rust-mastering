use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

pub fn sequentially_consistent_ordering() {
    let a = std::thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { 
                let ptr = std::ptr::addr_of_mut!(S);
                (*ptr).push('!');
            };
        }
    });

    let b = std::thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { 
                let ptr = std::ptr::addr_of_mut!(S);
                (*ptr).push('!');
            };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
