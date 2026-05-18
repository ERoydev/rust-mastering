use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicUsize, fence};

// Basically because i have weak and arc i would want to keep a counter for both.
// So we are going to have a Weak pointer when we use Arc::downgrade() ( Arc<T> -> Weak<T> ): "I know where this lives, but i'm not keeping it alive."
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

// ----------------- ARC
pub struct Arc<T> {
    weak: Weak<T>,
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        self.increment_data_ref_count_via_try_update();
        Arc { weak }
    }
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
    fn increment_data_ref_count_via_try_update(&self) {
        self.weak
            .data()
            .data_ref_count
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x == usize::MAX { None } else { Some(x + 1) }
            })
            .expect("data ref count overflow");
    }

    /// returns mutable-reference
    fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Taking &mut Self is ensuring I can borrow a value mutably only once. So i cannot try to borrow as mutable more than once.
        if arc.weak.data().alloc_ref_count.load(Ordering::Relaxed) == 1 {
            // I have to have only one ref so that's why i check for both Arc and Weak refs.
            fence(Ordering::Acquire);
            // Safety: Nothing else can access the data, since
            // there's only one Arc, to which we have exclusive access,
            // and no Weak pointers.
            let arc_data = unsafe { arc.weak.ptr.as_mut() };
            let option = arc_data.data.get_mut();
            // We know the data is still available since we
            // have an Arc to it, so this won't panic.
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
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

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // Dropping Arc should decrement both counters. When i drop Arc it automatically is going to result in dropping a Weak.
        if self
            .weak
            .data()
            .data_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);
            let ptr = self.weak.data().data.get();
            // Safety: The data reference counter is zero,
            // so nothing will access it.
            unsafe {
                *ptr = None;
            }
        }
    }
}

// ---------------- WEAK

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

    fn upgrade(&self) -> Option<Arc<T>> {
        // Basically I can upgrade ONLY if the data still exists, if there are only weak pointers left, there's no data left that can be shared through an Arc.
        // The indicator of existing data is the `strong_ref_counter`
        let mut current_strong_ref_count = self.data().data_ref_count.load(Ordering::Relaxed);

        loop {
            if current_strong_ref_count == 0 {
                return None;
            }
            assert!(current_strong_ref_count < usize::MAX);
            if let Err(actual) = self.data().data_ref_count.compare_exchange_weak(
                current_strong_ref_count,
                current_strong_ref_count + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                current_strong_ref_count = actual;
                continue;
            }

            return Some(Arc { weak: self.clone() });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        self.increment_ref_counter_via_try_update();
        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);

            // Box::from_raw is the standard safe way to reclaim ownership from raw pointer, when it goes out of scope it triggers the destructor
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_new() {
        let arc_data = Arc::new(15);
    }

    #[test]
    fn test() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        // Create an Arc with two weak pointers.
        let x = Arc::new(("hello", DetectDrop));
        let y = Arc::downgrade(&x);
        let z = Arc::downgrade(&x);

        let t = std::thread::spawn(move || {
            // Weak pointer should be upgradable at this point.
            let y = y.upgrade().unwrap();
            assert_eq!(y.0, "hello");
        });
        assert_eq!(x.0, "hello");
        t.join().unwrap();

        // The data shouldn't be dropped yet,
        // and the weak pointer should be upgradable.
        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);
        assert!(z.upgrade().is_some());

        drop(x);

        // Now, the data should be dropped, and the
        // weak pointer should no longer be upgradable.
        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
        assert!(z.upgrade().is_none());
    }
}
