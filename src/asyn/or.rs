extern crate alloc;
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct OrFuture<T> {
    pub(super) main: Pin<Box<dyn Future<Output = T>>>,
    pub(super) second: Pin<Box<dyn Future<Output = ()>>>,
}

impl<T> Future for OrFuture<T> {
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.main.as_mut().poll(ctx) {
            Poll::Ready(main) => Poll::Ready(Some(main)),
            Poll::Pending => match self.second.as_mut().poll(ctx) {
                Poll::Ready(_) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
