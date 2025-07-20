extern crate alloc;

use alloc::sync::Arc;

use crossbeam_queue::ArrayQueue;
use log::info;

use super::{checksum, Address, Packet, Protocol};
use crate::{asyn, network::arp, network::ethernet};

pub struct Socket {
    pub(super) protocol: Protocol,
    pub(super) address: Address,
    pub(super) recv_queue: Arc<ArrayQueue<Packet>>,
    pub(super) ethernet: Arc<ethernet::Socket>,
    pub(super) arp: Arc<arp::Service>,
}

impl Socket {
    pub async fn receive(&self) -> Packet {
        asyn::queue_pop(self.recv_queue.clone()).await
    }
    pub async fn send(&self, mut p: Packet) {
        p.set_protocol(self.protocol);
        if p.source_address() == Address([0; 4]) {
            p.set_source_address(&self.address);
        }

        p.set_header_checksum(0);
        p.set_header_checksum(checksum(p.header()));

        if p.eth.mac_destination() == ethernet::MacAddress([0; 6]) {
            match self.arp.lookup(&p.destination_address()).await {
                Some(a) => p.eth.set_mac_destination(a),
                None => (),
            };
        }

        if p.eth.mac_destination() != ethernet::MacAddress([0; 6]) {
            self.ethernet.send(p.eth);
        } else {
            info!("dropping packet without destination mac");
        }
    }
}
