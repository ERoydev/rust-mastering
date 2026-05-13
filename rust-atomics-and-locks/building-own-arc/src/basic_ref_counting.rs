use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::atomic::fence;

#[derive(Debug)]
#[allow(dead_code)]
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Arc<T> {
    // represents a pointer to T that is never null.
    ptr: NonNull<ArcData<T>>,
}

impl<T> Arc<T> {
    #[allow(dead_code)]
    pub fn new(data: T) -> Arc<T> {
        Arc {
            // Box::leak() consumes Box, prevents its destructor, leave the allocation alive for the rest of the program
            //  and returns a static mutable ref to that data
            //  LINK - https://softwaremill.com/leaking-memory-on-purpose-in-rust/
            // Then I convert the leaked mutable reference to a NonNull<T> (non-null raw pointer), guaranteed to never be null
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

    // returns mutable-reference
    fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Taking &mut Self is ensuring I can borrow a value mutably only once. So i cannot try to borrow as mutable more than once.
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            // Safety: Nothing else can access the data, since there's only one Arc,
            // to which we have exclusive access.
            unsafe { Some(&mut arc.ptr.as_mut().data) }
        } else {
            None
        }
        // The returned mutable ref borrows the lifetime from the arg, meaning nothing can use the original Arc as long as the returned (&mut T is still around), allowing for safe mutation.
    }

    // Bellow I have the same functionality but using `try_update()` to avoid this loop myself
    #[allow(dead_code)]
    pub fn increment_ref_counter_using_compare_exchange_weak(&self) {
        let mut ref_counter = self.data().ref_count.load(Ordering::Relaxed);
        loop {
            if ref_counter == usize::MAX {
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
    #[warn(unused)]
    pub fn increment_ref_counter_via_try_update(&self) {
        self.data()
            .ref_count
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                if x == usize::MAX { None } else { Some(x + 1) }
            })
            .expect("ref counter overflow");
    }
}

// Basically same as `data()`, but instead this allows me to get the inner data directly, without knowing about ArcData at all.
impl<T> Deref for Arc<T> {
    type Target = T;

    // Expose the inner T so Arc<T> can be used like &T.
    //
    // How auto-deref works in practice (e.g. `arc_data.name`):
    //  1. `arc_data` is `Arc<ArcExperimentalData>`
    //  2. You write `arc_data.name`
    //  3. Compiler checks: does `Arc<ArcExperimentalData>` have a field `name`? No.
    //  4. Compiler checks: does `Arc<ArcExperimentalData>` implement `Deref`? Yes.
    //  5. Compiler calls `deref()`, which returns `&ArcExperimentalData`
    //  6. Compiler checks: does `ArcExperimentalData` have a field `name`? Yes. Done.
    fn deref(&self) -> &T {
        &self.data().data
    }
}

// On .clone I should just increment the counter there is no "real" Clone of the ArcData
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.increment_ref_counter_via_try_update();
        Arc { ptr: self.ptr } // just return Arc with the same pointer
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire); // helps me catch the memory ordering
            // When I want to deallocate data from raw-pointer that is leaked I need to establish ownership from raw-pointer

            // Box::from_raw is the standard safe way to reclaim ownership from raw pointer, when it goes out of scope it triggers the destructor
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) })
        };
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
        assert_eq!(
            arc_data.name,
            "Daka".to_string(),
            "Name should be initialized to Daka"
        );
        assert_eq!(arc_data.age, 25, "Age should be initialied to 25");
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

    #[test]
    fn book_written_test() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        // Create two Arcs sharing an object containing a string
        // and a DetectDrop, to detect when it's dropped.
        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();

        // Send x to another thread, and use it there.
        let t = std::thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });

        // In parallel, y should still be usable here.
        assert_eq!(y.0, "hello");

        // Wait for the thread to finish.
        t.join().unwrap();

        // One Arc, x, should be dropped by now.
        // We still have y, so the object shouldn't have been dropped yet.
        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

        // Drop the remaining `Arc`.
        drop(y);

        // Now that `y` is dropped too,
        // the object should've been dropped.
        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    }

    // Run with:
    // MIRIFLAGS="-Zmiri-backtrace=full" cargo +nightly miri test miri_stress_clone_drop -- --nocapture
    #[test]
    #[cfg(miri)]
    fn miri_stress_clone_drop() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        println!("miri_stress_clone_drop: starting");

        for i in 0..500 {
            NUM_DROPS.store(0, Ordering::Relaxed);

            let a = Arc::new((42usize, DetectDrop));
            let b = a.clone();

            let t = std::thread::spawn(move || {
                assert_eq!(a.0, 42);
            });

            assert_eq!(b.0, 42);
            t.join().unwrap();

            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);
            drop(b);
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);

            if i % 100 == 0 {
                println!("miri_stress_clone_drop: iteration {}", i);
            }
        }

        println!("miri_stress_clone_drop: done");
    }

    // Run with different schedules:
    // for s in 1 2 3 4 5; do MIRIFLAGS="-Zmiri-seed=$s -Zmiri-backtrace=full" cargo +nightly miri test miri_stress_parallel_clones -- --exact || break; done
    #[test]
    #[cfg(miri)]
    fn miri_stress_parallel_clones() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        println!("miri_stress_parallel_clones: starting");

        for i in 0..200 {
            NUM_DROPS.store(0, Ordering::Relaxed);

            let base = Arc::new(("hello", DetectDrop));
            let mut handles = Vec::new();

            for _ in 0..4 {
                let c = base.clone();
                handles.push(std::thread::spawn(move || {
                    assert_eq!(c.0, "hello");
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);
            drop(base);
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);

            if i % 50 == 0 {
                println!("miri_stress_parallel_clones: iteration {}", i);
            }
        }

        println!("miri_stress_parallel_clones: done");
    }
}
