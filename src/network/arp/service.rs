extern crate alloc;

use super::{HardwareType, Operation, Packet};
use crate::{
    asyn,
    network::{ethernet, ip},
};
use alloc::sync::Arc;
use hashbrown::HashMap;

pub struct Service {
    pub ip: ip::Address,
    pub socket: ethernet::Socket,
    pub mac: ethernet::MacAddress,
    pub table: asyn::Mutex<HashMap<ip::Address, ethernet::MacAddress>>,
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
            table: asyn::Mutex::new(HashMap::new()),
        }
    }
    pub async fn lookup(&self, addr: &ip::Address) -> Option<ethernet::MacAddress> {
        let table = self.table.lock().await;

        match table.get(addr).copied() {
            Some(a) => Some(a),
            None => {
                // TODO: wait for response
                let mut request_raw = ethernet::Packet::new();
                request_raw.set_mac_destination(ethernet::MAC_BROADCAST);
                request_raw.set_size(28);
                let mut request = Packet(request_raw.data_mut());
                request.set_hardware_type(HardwareType::ETHERNET);
                request.set_protocol_type(ethernet::Type::IPV4);
                request.set_hardware_len(6);
                request.set_protocol_len(4);
                request.set_operation(Operation::REQUEST);
                request.set_sender_hardware_address(&self.mac);
                request.set_sender_protocol_address(&self.ip);
                request.set_target_hardware_address(&ethernet::MAC_BROADCAST);
                request.set_target_protocol_address(addr);

                self.socket.send(request_raw);

                None
            }
        }
    }

    pub fn start(self: Arc<Self>, e: Arc<dyn asyn::Executor>) {
        e.spawn(asyn::Task::new(self.task_receive()));
    }

    async fn task_receive(self: Arc<Self>) {
        loop {
            let mut received_raw = self.socket.receive().await;
            let received = Packet(received_raw.data_mut());

            if received.hardware_type() != HardwareType::ETHERNET {
                continue;
            }
            if received.protocol_type() != ethernet::Type::IPV4 {
                continue;
            }

            let mut table = self.table.lock().await;
            table.insert(
                received.sender_protocol_address(),
                received.sender_hardware_address(),
            );

            if received.operation() != Operation::REQUEST {
                continue;
            }

            if received.target_protocol_address() == self.ip {
                let mut response_raw = ethernet::Packet::new();
                response_raw.set_mac_destination(received.sender_hardware_address());
                response_raw.set_size(28);
                let mut response = Packet(response_raw.data_mut());
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
