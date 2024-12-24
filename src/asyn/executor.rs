extern crate alloc;

use alloc::sync::Arc;
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

use super::{waker::NoOpWaker, Task};

pub trait Executor {
    fn spawn(&self, task: Task);
}

pub struct SimpleExecutor {
    task_queue: ArrayQueue<Task>,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: ArrayQueue::new(256),
        }
    }

    pub fn run(&self) {
        let waker = Waker::from(Arc::new(NoOpWaker {}));
        let mut ctx = Context::from_waker(&waker);
        loop {
            let mut task = match self.task_queue.pop() {
                Some(t) => t,
                None => return,
            };
            match task.poll(&mut ctx) {
                Poll::Ready(()) => {}
                Poll::Pending => self.spawn(task),
            }
        }
    }
}

impl Executor for SimpleExecutor {
    fn spawn(&self, task: Task) {
        self.task_queue.push(task).expect("task queue full");
    }
}
