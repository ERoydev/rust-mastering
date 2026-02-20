use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

pub fn progress_reporting_from_multiple_threads() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    thread::sleep(Duration::from_secs(1)); // mock to make it to take some time
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        // Main thread shows status updates
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 { break; }
            println!("Working... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    }); // automatically joins
    println!("Done!");
}