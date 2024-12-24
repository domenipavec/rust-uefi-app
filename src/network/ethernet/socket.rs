extern crate alloc;

use alloc::sync::Arc;

use crossbeam_queue::ArrayQueue;

use super::{Packet, Type};
use crate::asyn;

pub struct Socket {
    pub(super) protocol: Type,
    pub(super) recv_queue: Arc<ArrayQueue<Packet>>,
    pub(super) send_queue: Arc<ArrayQueue<Packet>>,
}

impl Socket {
    pub async fn receive(&self) -> Packet {
        asyn::queue_pop(self.recv_queue.clone()).await
    }
    pub fn send(&self, mut p: Packet) {
        p.set_ether_type(self.protocol);
        // TODO: push async in case queue is full
        self.send_queue.push(p).unwrap();
    }
}
