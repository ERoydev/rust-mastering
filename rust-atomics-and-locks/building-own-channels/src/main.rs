use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};
mod mutex_based_channel_1;
mod single_atomic;
mod unsafe_one_shot_channel_2;
mod type_safety_4;

fn main() {
    unsafe_one_shot_channel_2::unsafe_one_shot_channel();
}
