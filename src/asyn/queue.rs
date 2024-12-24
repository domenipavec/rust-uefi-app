extern crate alloc;

use alloc::sync::Arc;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;

pub fn queue_pop<T>(queue: Arc<ArrayQueue<T>>) -> PopFuture<T> {
    PopFuture { queue }
}

pub struct PopFuture<T> {
    queue: Arc<ArrayQueue<T>>,
}

impl<T> Future for PopFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match self.queue.pop() {
            Some(p) => Poll::Ready(p),
            None => Poll::Pending,
        }
    }
}
