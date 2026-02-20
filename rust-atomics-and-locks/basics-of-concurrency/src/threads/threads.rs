pub fn threads_in_rust() {
    let numbers = vec![1, 2, 3];

    // Ordinary thread
    // thread::spawn(move || {
    //     for n in &numbers {
    //         println!("{n}");
    //     }
    // }).join().unwrap();

    // Using the underlying std::thread::Builder allowing me to add configurations to the thread
    // let builder = std::thread::Builder::new().stack_size(32 * 1024);
    // let handler = builder
    //     .spawn(move || {
    //         for n in &numbers {
    //             println!("{n}");
    //         }
    //     })
    //     .unwrap()
    //     .join()
    //     .unwrap();

    // Scoped threads -> This will not outlive the scope of the closure we pass to that function, safely borrow local variables
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("lenght: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
        // When the scope ends all threads that haven't been joined yet are automatically joined.
    });

    println!("numbers: {:?}", numbers);
}
