use std::cell::UnsafeCell;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;

// Basically because i have weak and arc i would want to keep a counter for both.
struct ArcData<T> {
    /// Number of `Arc`s.
    data_ref_count: AtomicUsize,
    /// Number of `Arc`s and `Weak`s combined.
    alloc_ref_count: AtomicUsize,
    /// The data. `None` if there's only weak pointers left.
    // This allows us to drop the T while ArcData<T> is still in use by the weak pointers. So that way i can drop the Parent, without getting prevented by childs that reference it.
    data: UnsafeCell<Option<T>>, // UnsafeCell for Interior mutability, to allow that to happen when `ArcData<T>` isn't exclusively owned
}

pub struct Arc<T> {
    weak: Weak<T>,
}

// impl<T> Arc<T> {
//     pub fn new(data: T) -> Arc<T> {
//         Arc {
//             weak: Weak { ptr: Nonull },
//         }
//     }
// }

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_weak() {
        let some_value: std::rc::Weak<u32> = std::rc::Weak::new();

        println!("---- {:?}", some_value.upgrade());
    }
}
