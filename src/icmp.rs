use core::fmt;
use uefi_raw::newtype_enum;

use crate::ip;

pub struct Handler {}

impl Handler {
    pub fn handle(&mut self, sender: &mut ip::Handler, ip_packet: &mut ip::Packet) {
        if ip::compute_checksum(&ip_packet.data()) != 0 {
            return;
        }
        let source_ip = ip_packet.source_address();
        let packet = Packet(ip_packet.data());
        if packet.typ() == Type::ECHO_REQUEST {
            sender
                .send(source_ip, ip::Protocol::ICMP, |buf| {
                    let packet_buffer = &mut buf[..packet.0.len()];
                    packet_buffer.copy_from_slice(&packet.0);
                    let mut response = Packet(packet_buffer);
                    response.set_type(Type::ECHO_REPLY);
                    response.set_checksum(0);
                    response.set_checksum(ip::compute_checksum(&response.0));

                    packet.0.len().try_into().unwrap()
                })
                .unwrap();
        }
    }
}

pub struct Packet<'p>(pub &'p mut [u8]);

impl fmt::Debug for Packet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut binding = f.debug_struct("ICMPPacket");
        binding
            .field("type", &self.typ())
            .field("code", &self.code())
            .field("checksum", &self.checksum());
        if self.typ() == Type::ECHO_REQUEST || self.typ() == Type::ECHO_REPLY {
            binding
                .field("identifier", &self.identifier())
                .field("sequence_number", &self.sequence_number());
        }
        binding.field("data_len", &(self.0.len() - 8));
        binding.finish()
    }
}

impl Packet<'_> {
    fn typ(&self) -> Type {
        Type(self.0[0])
    }
    fn set_type(&mut self, t: Type) {
        self.0[0] = t.0
    }
    fn code(&self) -> u8 {
        self.0[1]
    }
    fn set_code(&mut self, c: u8) {
        self.0[1] = c
    }
    fn checksum(&self) -> u16 {
        u16::from_be_bytes(self.0[2..4].try_into().unwrap())
    }
    fn set_checksum(&mut self, s: u16) {
        self.0[2..4].clone_from_slice(&s.to_be_bytes());
    }
    fn identifier(&self) -> u16 {
        u16::from_be_bytes(self.0[4..6].try_into().unwrap())
    }
    fn set_identifier(&mut self, i: u16) {
        self.0[4..6].clone_from_slice(&i.to_be_bytes());
    }
    fn sequence_number(&self) -> u16 {
        u16::from_be_bytes(self.0[6..8].try_into().unwrap())
    }
    fn set_sequence_number(&mut self, n: u16) {
        self.0[6..8].clone_from_slice(&n.to_be_bytes());
    }
}

newtype_enum! {
    pub enum Type: u8 => {
        ECHO_REPLY = 0,
        DESTINATION_UNREACHABLE = 3,
        SOURCE_QUENCH = 4,
        REDIRECT_MESSAGE = 5,
        ECHO_REQUEST = 8,
        ROUTER_ADVERTISEMENT = 9,
        ROUTER_SOLICITATION = 10,
        TIME_EXCEEDED = 11,
        PARAMETER_PROBLEM = 12,
        TIMESTAMP = 13,
        TIMESTAMP_REPLY = 14,
        INFORMATION_REQUEST = 15,
        INFORMATION_REPLY = 16,
        ADDRESS_MASK_REQUEST = 17,
        ADDRESS_MASK_REPLY = 18,
        TRACEROUTE = 30,
        EXTENDED_ECHO_REQUEST = 42,
        EXTENDED_ECHO_REPLY = 43,
    }
}
