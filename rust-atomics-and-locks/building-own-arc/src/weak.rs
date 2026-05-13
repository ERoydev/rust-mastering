use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// Basically because i have weak and arc i would want to keep a counter for both.
struct ArcData<T> {
    /// Number of `Arc`s.
    data_ref_count: AtomicUsize,
    /// Number of `Arc`s and `Weak`s combined.
    alloc_ref_count: AtomicUsize,
    /// The data. `None` if there's only weak pointers left.
    // This allows us to drop the T while ArcData<T> is still in use by the weak pointers. So that way i can drop the Parent, without getting prevented by childs that reference it.
    // The data. `None` if there's only weak pointers left.
    data: UnsafeCell<Option<T>>, // UnsafeCell allow to us to make the data None whe there isn't exclusively owned.
}

pub struct Arc<T> {
    weak: Weak<T>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    data_ref_count: AtomicUsize::new(1),
                    alloc_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        self.increment_ref_counter_via_try_update();
        Weak { ptr: self.ptr }
    }
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}
unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    fn increment_ref_counter_via_try_update(&self) {
        self.data()
            .alloc_ref_count
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x == usize::MAX { None } else { Some(x + 1) }
            })
            .expect("alloc ref count overflow");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_new() {
        let arc_data = Arc::new(15);
    }
}
