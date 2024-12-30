use crate::{asyn, network::ip};

use super::{Packet, Type};

pub struct Service {
    socket: ip::Socket,
}

impl Service {
    pub fn new(ip: &mut ip::Service) -> Service {
        Service {
            socket: ip.open(ip::Protocol::ICMP),
        }
    }
    pub fn start(self, e: &dyn asyn::Executor) {
        e.spawn(asyn::Task::new(self.task_receive()));
    }

    async fn task_receive(self) {
        loop {
            let received = Packet {
                ip: self.socket.receive().await,
            };

            if ip::checksum(&received.ip.data()) != 0 {
                log::info!("Invalid icmp checksum");
                continue;
            }

            if received.typ() == Type::ECHO_REQUEST {
                let mut response = Packet {
                    ip: ip::Packet::new(),
                };
                response.set_data(received.data());

                response.set_type(Type::ECHO_REPLY);
                response.set_code(0);
                response.set_identifier(received.identifier());
                response.set_sequence_number(received.sequence_number());
                response.set_checksum(ip::checksum(response.ip.data()));

                response
                    .ip
                    .set_destination_address(&received.ip.source_address());

                response
                    .ip
                    .eth
                    .set_mac_destination(received.ip.eth.mac_source());

                self.socket.send(response.ip);
            }
        }
    }
}
