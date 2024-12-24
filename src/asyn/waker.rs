extern crate alloc;

use alloc::sync::Arc;
use alloc::task::Wake;

pub(super) struct NoOpWaker {}

impl Wake for NoOpWaker {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {}
}
