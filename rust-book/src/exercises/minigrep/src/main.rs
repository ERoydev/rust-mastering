
use std::{env, os::unix::process};

use minigrep::Config;

// Goal: cargo run -- searchstring example-filename.txt

// Test it with this command
// cargo run -- language /Users/emilemilovroydev/Rust/rust-mastering/rust-book/src/exercises/minigrep/test-file.txt

fn main() {
    // Accept ENV args into a vec
    let args: Vec<String> = env::args().collect(); 

    let config = match Config::new(&args){
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            std::process::exit(1);
        }
    };


    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
    };
}
