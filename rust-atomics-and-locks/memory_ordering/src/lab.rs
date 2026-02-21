
pub fn pointers() {
    let num = 42;
    let r: *const i32 = &num; // Raw-pointer

    unsafe {
        // I can only dereference raw-pointer in an unsafe block, 
        // because Rust compilation doesn't provide safety for using raw-pointers and can lead to dangling references
        println!("r: {:?}", *r);
    }
}