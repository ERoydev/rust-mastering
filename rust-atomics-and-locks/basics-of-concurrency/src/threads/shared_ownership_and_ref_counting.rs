// static item is owned by the program, never dropped, every thread can borrow it
pub static STATIC_GLOBAL: [u8; 3] = [1, 2, 3];

pub fn shared_own_and_ref_counting() {
    // Share ownership by leaking an allocation. `Box::leak()`
    // It releases the ownership of a Box, meaning this Box will live forever, without an owner, can be borrowed by an thread as long as the program runs.
    let x = Box::leak(Box::new([1, 2, 3]));

    std::thread::spawn(move || {
        dbg!(x);
    })
    .join()
    .unwrap();
}
