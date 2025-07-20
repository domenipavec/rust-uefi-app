use core::{
    cell::UnsafeCell,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
    status: AtomicUsize,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

pub struct MutexFuture<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(inner: T) -> Mutex<T> {
        Mutex {
            inner: UnsafeCell::new(inner),
            status: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> MutexFuture<T> {
        MutexFuture { mutex: self }
    }
}

impl<'a, T> Future for MutexFuture<'a, T> {
    type Output = MutexGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<MutexGuard<'a, T>> {
        match self
            .mutex
            .status
            .compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => Poll::Ready(MutexGuard { mutex: self.mutex }),
            Err(_) => Poll::Pending,
        }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.status.store(0, Ordering::Relaxed);
    }
}
