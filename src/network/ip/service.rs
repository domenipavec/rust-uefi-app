extern crate alloc;

use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;
use hashbrown::HashMap;

use crate::{asyn, network::ethernet};

use super::{checksum, Address, Packet, Protocol, Socket};

pub struct Service {
    ethernet: Arc<ethernet::Socket>,
    sockets: HashMap<Protocol, Arc<ArrayQueue<Packet>>>,
    address: Address,
}

impl Service {
    pub fn new(eth: &mut ethernet::Service, address: Address) -> Service {
        Service {
            ethernet: Arc::new(eth.open(ethernet::Type::IPV4)),
            sockets: HashMap::new(),
            address: address,
        }
    }

    pub fn start(self, e: &dyn asyn::Executor) {
        let arc = Arc::new(self);
        e.spawn(asyn::Task::new(arc.clone().task_receive()));
    }

    pub fn open(&mut self, p: Protocol) -> Socket {
        let s = Socket {
            protocol: p,
            address: self.address,
            recv_queue: Arc::new(ArrayQueue::new(16)),
            ethernet: self.ethernet.clone(),
        };
        self.sockets.insert(p, s.recv_queue.clone());
        s
    }

    async fn task_receive(self: Arc<Self>) {
        loop {
            let ip_packet = Packet {
                eth: self.ethernet.receive().await,
            };
            if checksum(ip_packet.header()) != 0 {
                log::info!("Invalid ip checksum");
                continue;
            }

            if let Some(s) = self.sockets.get(&ip_packet.protocol()) {
                s.push(ip_packet).unwrap();
            }
        }
    }
}
