extern crate alloc;

use alloc::boxed::Box;
use core::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

type PinFuture = Pin<Box<dyn Future<Output = ()>>>;

pub struct Task {
    future: PinFuture,
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Task")
    }
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    pub(super) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
