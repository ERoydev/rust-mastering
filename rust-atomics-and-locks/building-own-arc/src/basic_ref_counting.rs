use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::atomic::fence;

#[derive(Debug)]
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

#[derive(Debug)]
pub struct Arc<T> {
    // represents a pointer to T that is never null.
    ptr: NonNull<ArcData<T>>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            // Box::leak() consumes Box, prevents its destructor, leave the allocation alive for the rest of the program
            //  and retunrs a static mutable ref to that data
            //  LINK - https://softwaremill.com/leaking-memory-on-purpose-in-rust/
            // Then i convert the leaked mutable reference to a NonNull<T> (non-null raw pointer), guaranteed to never be null
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    // returns a shared-reference (private method)
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    // Bellow i have the same functionality but using `try_update()` to avoid this loop myself
    pub fn increment_ref_counter_using_compare_exchange_weak(&self) {
        let mut ref_counter = self.data().ref_count.load(Ordering::Relaxed);
        loop {
            if ref_counter >= usize::MAX {
                std::process::abort()
            }
            match self.data().ref_count.compare_exchange_weak(
                ref_counter,
                ref_counter + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(current) => ref_counter = current,
            }
        }
    }

    // This is the same as the `compare_exchange`, its just a wrapper so i don't write the loop myself.
    // `update` and `try_update` are from rust 1.95 newer release
    pub fn increment_ref_counter_via_try_update(&self) {
        self.data()
            .ref_count
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x >= usize::MAX { None } else { Some(x + 1) }
            })
            .expect("ref counter overflow");
    }
}

// Basically same as `data()`, but instead this allows me to get the inner data directly, without knowing about ArcData at all.
impl<T> Deref for Arc<T> {
    type Target = T;

    // Expose the inner T so Arc<T> can be used like &T.
    fn deref(&self) -> &T {
        &self.data().data
    }
}

// On clone i should just increment the counter there is no "real" Clone of the ArcData
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.increment_ref_counter_via_try_update();
        Arc { ptr: self.ptr } // just return Arc with the same pointer
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let prev_val = self.data().ref_count.fetch_sub(1, Ordering::Release);

        if prev_val == 1 {
            fence(Ordering::Acquire); // helps me catch the memory ordering
            // When i want to deallocate data from raw-pointer that is leaked i need to establish ownership from raw-pointer

            // Box::from_raw is the standard safe way to reclaim ownership from raw pointer, when it goes out of scope it triggers the destructor
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) })
        }
    }
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug)]
    struct ArcExperimentalData {
        name: String,
        age: u8,
    }

    struct DropTracker;

    impl Drop for DropTracker {
        fn drop(&mut self) {
            DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn setup() -> Arc<ArcExperimentalData> {
        Arc::new(ArcExperimentalData {
            name: "Daka".to_string(),
            age: 25,
        })
    }

    // cargo test test_arc_new -- --nocapture
    #[test]
    fn test_arc_new() {
        println!();

        let arc_data = setup();
        println!("Arc data is holding a raw pointer: {:?}", arc_data);

        println!(
            "Dereference the raw pointer via `.as_ref()` and return a shared reference to the data behind the pointer {:?}",
            arc_data.data()
        );

        println!();
    }

    #[test]
    fn test_clone_ref_counter_increment() {
        println!();

        let arc_data = setup();
        let v = arc_data.data().ref_count.load(Ordering::Relaxed);
        assert_eq!(v, 1, "Ref counter must be 1");
        let cloned_arc_data = arc_data.clone();
        let v2 = cloned_arc_data.data().ref_count.load(Ordering::Relaxed);
        assert_eq!(v2, 2, "Ref counter must be 2");

        assert_eq!(
            arc_data.ptr, cloned_arc_data.ptr,
            "Pointers of cloned Arc must be identical"
        );

        println!();
    }

    #[test]
    fn test_deref() {
        let arc_data = setup();

        // Field access triggers auto-deref from Arc<T> to T.
        assert_eq!(arc_data.name, "Daka");
        assert_eq!(arc_data.age, 25);

        // Same result with explicit deref syntax.
        assert_eq!((*arc_data).age, 25);
    }

    #[test]
    #[ignore = "Intentionally aborts process to demonstrate overflow handling"]
    fn test_ref_counter_overflow_abort() {
        let arc_data = setup();

        arc_data
            .data()
            .ref_count
            .store(usize::MAX, Ordering::Relaxed);

        // This clone should trigger overflow handling and abort the process.
        let _ = arc_data.clone();
    }

    // cargo test test_drop_on_last_ref_counter -- --nocapture
    #[test]
    fn test_drop_on_last_ref_counter() {
        DROP_COUNTER.store(0, Ordering::Relaxed);

        let arc_data = Arc::new(DropTracker);
        let cloned = arc_data.clone();

        assert_eq!(DROP_COUNTER.load(Ordering::Relaxed), 0);

        drop(arc_data);
        assert_eq!(
            DROP_COUNTER.load(Ordering::Relaxed),
            0,
            "Inner data must not be dropped while another Arc still exists"
        );

        drop(cloned);
        assert_eq!(
            DROP_COUNTER.load(Ordering::Relaxed),
            1,
            "Inner data must be dropped exactly once on the last Arc drop"
        );
    }
}
