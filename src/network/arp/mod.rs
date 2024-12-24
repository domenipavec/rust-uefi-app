use super::{ethernet, ip};
use crate::asyn::{Executor, Task};

mod packet;
use packet::{HardwareType, Operation, Packet};

pub struct Service {
    pub ip: ip::Address,
    pub socket: ethernet::Socket,
    pub mac: ethernet::MacAddress,
}

impl Service {
    pub fn new(
        ip: ip::Address,
        mac: ethernet::MacAddress,
        service: &mut ethernet::Service,
    ) -> Service {
        Service {
            ip: ip,
            socket: service.open(ethernet::Type::ARP),
            mac: mac,
        }
    }
    pub fn start(self, e: &dyn Executor) {
        e.spawn(Task::new(self.task_receive()));
    }

    async fn task_receive(self) {
        loop {
            let mut received_raw = self.socket.receive().await;
            let received = Packet(received_raw.data());

            if received.hardware_type() != HardwareType::ETHERNET {
                continue;
            }
            if received.operation() != Operation::REQUEST {
                continue;
            }
            if received.protocol_type() != ethernet::Type::IPV4 {
                continue;
            }

            if received.target_protocol_address() == self.ip {
                let mut response_raw = ethernet::Packet::new();
                response_raw.set_mac_source(self.mac);
                response_raw.set_mac_destination(received.sender_hardware_address());
                response_raw.set_size(28);
                let mut response = Packet(response_raw.data());
                response.set_hardware_type(HardwareType::ETHERNET);
                response.set_protocol_type(ethernet::Type::IPV4);
                response.set_hardware_len(6);
                response.set_protocol_len(4);
                response.set_operation(Operation::RESPONSE);
                response.set_sender_hardware_address(&self.mac);
                response.set_sender_protocol_address(&self.ip);
                response.set_target_hardware_address(&received.sender_hardware_address());
                response.set_target_protocol_address(&received.sender_protocol_address());

                self.socket.send(response_raw);
            }
        }
    }
}
