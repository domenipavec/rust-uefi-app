extern crate alloc;

use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;

use super::{Packet, Protocol, Service};
use crate::asyn;

pub struct Socket {
    pub(super) service: Arc<Service>,
    pub(super) protocol: Protocol,
    pub(super) recv_queue: Arc<ArrayQueue<Packet>>,
}

impl Socket {
    pub async fn receive(&self) -> Packet {
        asyn::queue_pop(self.recv_queue.clone()).await
    }

    pub async fn send(&self, mut p: Packet) {
        p.set_protocol(self.protocol);
        self.service.send(p).await;
    }
}
