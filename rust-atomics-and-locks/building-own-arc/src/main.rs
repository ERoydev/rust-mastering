mod basic_ref_counting;
mod weak;
mod optimizing;

fn main() {
    println!("Hello, world! From Building own Arc");
    // cargo test -- --nocapture
    // cargo +nightly miri test -> to run it with miri
}
