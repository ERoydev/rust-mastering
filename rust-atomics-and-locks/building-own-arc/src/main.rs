mod basic_ref_counting;

fn main() {
    println!("Hello, world! From Building own Arc");
    // cargo test -- --nocapture
    // cargo +nightly miri test -> to run it with miri
}
