use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

pub fn id_allocation_5() {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed); // This can have overflow issues tipically
}

// This is the safest way, Typically we only use compare_exchange instead of (fetch_add, fetch_sub)
pub fn id_allocation_safe() {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut current = NEXT_ID.load(Relaxed);

    // The loop idea is that if someone has already changed this current value, instead i will return that, without doing `store` in the memory
    loop {
        let new = current + 1;
        match NEXT_ID.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v
        };
    }
}
// This is better approach since compare_exchange is doing, (load, comparison and store)
pub fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);

    loop {
        let new: u32 = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(t) => return,
            Err(v) => current = v
        }
    }

}