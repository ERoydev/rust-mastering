use std::{sync::atomic::AtomicUsize, thread, time::Duration};

pub fn progress_reporting() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
                main_thread.unpark(); // wake up the main thread, send notification
            }
        });

        // Main thread shows status updates
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 { break; }
            println!("Working... {n}/100 done");
            std::thread::park_timeout(Duration::from_secs(1));
        }
    }); // automatically joins
    println!("Done!");
}