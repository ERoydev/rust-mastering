use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};
mod blocking_6;
mod borrowing_5;
mod mutex_based_channel_1;
mod single_atomic;
mod type_safety_4;
mod unsafe_one_shot_channel_2;

fn main() {
    // unsafe_one_shot_channel_2::unsafe_one_shot_channel();
    type_safety_4::type_safety();
}
