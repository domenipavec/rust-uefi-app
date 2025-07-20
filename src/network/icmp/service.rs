extern crate alloc;

use crossbeam_queue::ArrayQueue;
use hashbrown::HashMap;

use alloc::sync::Arc;
use log::info;

use crate::{asyn, network::ip};

use super::{Packet, Socket, Type};

pub struct Service {
    next_request_identifier: u16,
    ip_socket: Arc<ip::Socket>,
    sockets: HashMap<(ip::Address, u16), Arc<ArrayQueue<Packet>>>,
}

impl Service {
    pub fn new(ip: &mut ip::Service) -> Service {
        Service {
            ip_socket: Arc::new(ip.open(ip::Protocol::ICMP)),
            sockets: HashMap::new(),
            next_request_identifier: 0,
        }
    }
    pub fn start(self, e: &dyn asyn::Executor) {
        e.spawn(asyn::Task::new(self.task_receive()));
    }

    pub fn open(&mut self, ip_address: ip::Address) -> Socket {
        let s = Socket {
            identifier: self.next_request_identifier,
            sequence: 0,
            ip_address: ip_address,
            recv_queue: Arc::new(ArrayQueue::new(16)),
            ip_socket: self.ip_socket.clone(),
        };
        self.next_request_identifier += 1;
        self.sockets
            .insert((ip_address, s.identifier), s.recv_queue.clone());
        s
    }

    async fn task_receive(self) {
        loop {
            let received = Packet {
                ip: self.ip_socket.receive().await,
            };

            if ip::checksum(&received.ip.data()) != 0 {
                log::info!("Invalid icmp checksum");
                continue;
            }

            match received.typ() {
                Type::ECHO_REQUEST => {
                    let mut response = Packet::new();
                    response.set_data(received.data());

                    response.set_type(Type::ECHO_REPLY);
                    response.set_code(0);
                    response.set_identifier(received.identifier());
                    response.set_sequence_number(received.sequence_number());
                    response.set_checksum(ip::checksum(response.ip.data()));

                    response
                        .ip
                        .set_destination_address(&received.ip.source_address());

                    self.ip_socket.send(response.ip).await;
                }
                Type::ECHO_REPLY => match self
                    .sockets
                    .get(&(received.ip.source_address(), received.identifier()))
                {
                    Some(q) => q.push(received).expect("reply queue full"),
                    None => info!(
                        "icmp reply from unrequested ip and identifier {:?}, {}",
                        received.ip.source_address(),
                        received.identifier(),
                    ),
                },
                _ => info!("unknown icmp type received {:?}", received.typ()),
            }
        }
    }
}
