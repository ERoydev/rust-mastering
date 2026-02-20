use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;


pub fn load_and_store() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work.
    let background_thread = std::thread::spawn(|| {
        while !STOP.load(Relaxed) { // reads the curr value from memory in an atomic way. Get most up-to-date value written by any thread
            // some_work();
            println!("Some Work");
        }
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    println!("Reached");
    // Inform the background thread it needs to stop.
    STOP.store(true, Relaxed); // writes the value true to stop flag in memory atomically, so any thread reading it will see a valid value

    // Wait unit the background thread finishes.
    background_thread.join().unwrap();
}