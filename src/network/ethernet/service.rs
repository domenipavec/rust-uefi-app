extern crate alloc;

use alloc::sync::Arc;
use core::panic;
use crossbeam_queue::ArrayQueue;
use hashbrown::HashMap;

use super::{ether_type::Type, simple_network, MacAddress, Packet, Socket};
use crate::asyn::{self, Executor, Task};

pub struct Service {
    network: simple_network::SimpleNetwork,
    sockets: HashMap<Type, Arc<ArrayQueue<Packet>>>,
    send_queue: Arc<ArrayQueue<Packet>>,
}

impl Service {
    pub fn new() -> Service {
        Service {
            network: simple_network::SimpleNetwork::new(),
            sockets: HashMap::new(),
            send_queue: Arc::new(ArrayQueue::new(16)),
        }
    }

    pub fn start(self, e: &dyn Executor) {
        let arc = Arc::new(self);
        e.spawn(Task::new(arc.clone().task_receive()));
        e.spawn(Task::new(arc.clone().task_send()));
    }

    pub fn mac_address(&self) -> MacAddress {
        self.network.mac_address()
    }

    pub fn open(&mut self, p: Type) -> Socket {
        let s = Socket {
            protocol: p,
            recv_queue: Arc::new(ArrayQueue::new(16)),
            send_queue: self.send_queue.clone(),
        };
        self.sockets.insert(p, s.recv_queue.clone());
        s
    }

    async fn task_send(self: Arc<Self>) {
        loop {
            let p = asyn::queue_pop(self.send_queue.clone()).await;
            if let Err(e) = self.network.transmit(&p.data[..p.header_size() + p.size()]) {
                panic!("{}", e);
            }
        }
    }

    async fn task_receive(self: Arc<Self>) {
        loop {
            let mut p = Packet::new();

            let usize = match self.network.receive(p.data.as_mut()).await {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };
            p.set_size(usize);

            if let Some(s) = self.sockets.get(&p.ether_type()) {
                s.push(p).unwrap();
            }
        }
    }
}
