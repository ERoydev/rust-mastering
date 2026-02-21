mod happens_before_relationship;
mod spawning_and_joining;
mod relaxed_ordering;
mod release_acquire;
mod example_locking;
mod lazy_initialization_with_indirection;
mod lab;
mod sequentially_consistent_ordering;
mod fences;
mod simple_fence_example;
/*
Memory ordering - Reordering and Optimizations

The compiler and CPU can reorder the code we write if they think that will lead to faster execution.
But it will only do this if this doesn't change the way that the program behaves from the perspective of a single thread. 

*/
fn f(a: &mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;
}

fn main() {
    // lab::pointers();
    // happens_before_relationship::happens_before_relationship();
    // spawning_and_joining::spawning_and_joining();
    // relaxed_ordering::relaxed_ordering();
    // release_acquire::release_acquire();
    // example_locking::example_locking();
    // lazy_initialization_with_indirection::lazy_initialization_with_indirection();
    // sequentially_consistent_ordering::sequentially_consistent_ordering();
    // fences::fences();
    simple_fence_example::simple_fence_example();
}
