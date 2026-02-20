pub mod load_and_store_1;
pub mod progress_reporting;
pub mod progress_reporting_from_multiple_threads;
pub mod progress_reporting_multiple_threads_part_2;
pub mod id_allocation_5;

fn main() {
    // load_and_store::load_and_store();
    // progress_reporting::progress_reporting();
    // progress_reporting_from_multiple_threads::progress_reporting_from_multiple_threads();
    // progress_reporting_multiple_threads_part_2::progress_reporting_multiple_threads_part_2();

    let data = vec![1, 2, 3];

    let poin = data.as_ptr();

    println!("Po: {:?}", poin)
}
