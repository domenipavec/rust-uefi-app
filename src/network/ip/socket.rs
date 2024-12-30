extern crate alloc;

use alloc::sync::Arc;

use crossbeam_queue::ArrayQueue;

use super::{checksum, Address, Packet, Protocol};
use crate::{asyn, network::ethernet};

pub struct Socket {
    pub(super) protocol: Protocol,
    pub(super) address: Address,
    pub(super) recv_queue: Arc<ArrayQueue<Packet>>,
    pub(super) ethernet: Arc<ethernet::Socket>,
}

impl Socket {
    pub async fn receive(&self) -> Packet {
        asyn::queue_pop(self.recv_queue.clone()).await
    }
    pub fn send(&self, mut p: Packet) {
        p.set_protocol(self.protocol);
        if p.source_address() == Address([0; 4]) {
            p.set_source_address(&self.address);
        }

        p.set_header_checksum(0);
        p.set_header_checksum(checksum(p.header()));

        if p.eth.mac_destination() == ethernet::MacAddress([0; 6]) {
            // TODO: arp lookup
            // p.eth.set_mac_destination(d);
        }

        self.ethernet.send(p.eth);
    }
}
