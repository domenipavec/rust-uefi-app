extern crate alloc;

use alloc::sync::Arc;

use crossbeam_queue::ArrayQueue;

use super::{Packet, Type};
use crate::{asyn, network::ip};

pub struct Socket {
    pub(super) identifier: u16,
    pub(super) sequence: u16,
    pub(super) ip_address: ip::Address,
    pub(super) recv_queue: Arc<ArrayQueue<Packet>>,
    pub(super) ip_socket: Arc<ip::Socket>,
}

impl Socket {
    pub async fn receive(&self, t: f64) -> Option<Packet> {
        asyn::queue_pop_timeout(self.recv_queue.clone(), t).await
    }

    pub async fn send(&self, data: &[u8]) {
        let mut request = Packet::new();

        request.set_data(data);

        request.set_type(Type::ECHO_REQUEST);
        request.set_code(0);
        request.set_identifier(self.identifier);
        request.set_sequence_number(self.sequence);
        request.set_checksum(ip::checksum(request.ip.data()));

        request.ip.set_destination_address(&self.ip_address);

        self.ip_socket.send(request.ip).await;
    }
}
