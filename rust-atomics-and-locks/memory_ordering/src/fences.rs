use std::{sync::atomic::{AtomicBool, Ordering::{Acquire, Release, SeqCst, Relaxed}, fence}, time::Duration};

// non-atomic shared variable
static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

pub fn fences() {
    for i in 0..10 {
        std::thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i as usize] = data };
            READY[i as usize].store(true, Release)
        });
    }

    std::thread::sleep(Duration::from_millis(500));
    // The thing is that i load 10 times without need ing to use `acquire-load` operations to read the booleans and i just use `Relaxed`
    // And i use the `acquire-fence` before reading the data only once, but only if i have a data to be read.
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed)); // calls the closure 10 times - once for each index 0..9
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }
}

// The whole idea is that this could be completely unnecessary to put effort into such optimization
// but this pattern allows me to save the overhead of additional acquire operations when building highly efficient concurrent data structures.


fn some_calculation(idx: i32) -> u64 {
    let mut arr = [0u64; 10];
    for i in 0..10 {
        arr[i] = idx as u64 + i as u64;
    }
    arr[idx as usize]
}