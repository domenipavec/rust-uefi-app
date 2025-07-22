extern crate alloc;

use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;
use hashbrown::HashMap;
use log::info;

use super::{checksum, Address, Packet, Protocol, Socket};
use crate::{asyn, network::arp, network::ethernet};

pub struct Service {
    ethernet: Arc<ethernet::Socket>,
    arp_service: Arc<arp::Service>,

    sockets: asyn::Mutex<HashMap<Protocol, Arc<ArrayQueue<Packet>>>>,

    address: Address,
    netmask: Address,
    gateway: Address,
}

impl Service {
    pub fn new(
        eth: &mut ethernet::Service,
        arp: Arc<arp::Service>,
        address: Address,
        netmask: Address,
        gateway: Address,
    ) -> Service {
        Service {
            ethernet: Arc::new(eth.open(ethernet::Type::IPV4)),
            arp_service: arp,

            sockets: asyn::Mutex::new(HashMap::new()),

            address: address,
            netmask: netmask,
            gateway: gateway,
        }
    }

    pub fn start(self: Arc<Self>, e: Arc<dyn asyn::Executor>) {
        e.spawn(asyn::Task::new(self.task_receive()));
    }

    pub async fn open(self: Arc<Self>, p: Protocol) -> Socket {
        let s = Socket {
            protocol: p,
            recv_queue: Arc::new(ArrayQueue::new(16)),
            service: self.clone(),
        };
        let mut sockets = self.sockets.lock().await;
        sockets.insert(p, s.recv_queue.clone());
        s
    }

    pub(super) async fn send(&self, mut p: Packet) {
        if p.source_address() == Address([0; 4]) {
            p.set_source_address(&self.address);
        }

        p.set_header_checksum(0);
        p.set_header_checksum(checksum(p.header()));

        if p.eth.mac_destination() == ethernet::MacAddress([0; 6]) {
            if p.destination_address() & self.netmask == self.address & self.netmask {
                match self.arp_service.lookup(&p.destination_address()).await {
                    Some(a) => p.eth.set_mac_destination(a),
                    None => (),
                };
            } else {
                match self.arp_service.lookup(&self.gateway).await {
                    Some(a) => p.eth.set_mac_destination(a),
                    None => (),
                };
            }
        }

        if p.eth.mac_destination() != ethernet::MacAddress([0; 6]) {
            self.ethernet.send(p.eth);
        } else {
            info!("dropping packet without destination mac");
        }
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

            let sockets = self.sockets.lock().await;
            if let Some(s) = sockets.get(&ip_packet.protocol()) {
                s.push(ip_packet).unwrap();
            }
        }
    }
}
