use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicBool,
    thread,
};

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    fn lock(&self) -> Guard<'_, T> {
        while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            spin_loop();
        }

        Guard { lock: self }
    }
}

struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

unsafe impl<T: Send> Sync for Guard<'_, T> {}
unsafe impl<T: Sync> Send for Guard<'_, T> {}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

fn main() {
    let lock = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            lock.lock().push(1);
        });

        s.spawn(|| {
            let mut g = lock.lock();
            g.push(2);
            g.push(3);
        });
    });

    let g = lock.lock();

    println!("{:?}", g.as_slice());
}
