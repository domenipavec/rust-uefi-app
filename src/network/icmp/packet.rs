use core::fmt;

use uefi_raw::newtype_enum;

use crate::network::ip;

pub struct Packet {
    pub ip: ip::Packet,
}

impl fmt::Debug for Packet {
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
        binding.field("data_len", &(self.data().len() - 8));
        binding.finish()
    }
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            ip: ip::Packet::new(),
        }
    }

    pub fn typ(&self) -> Type {
        Type(self.ip.data()[0])
    }
    pub fn set_type(&mut self, t: Type) {
        self.ip.data_mut()[0] = t.0
    }
    pub fn code(&self) -> u8 {
        self.ip.data()[1]
    }
    pub fn set_code(&mut self, c: u8) {
        self.ip.data_mut()[1] = c
    }
    pub fn checksum(&self) -> u16 {
        u16::from_be_bytes(self.ip.data()[2..4].try_into().unwrap())
    }
    pub fn set_checksum(&mut self, s: u16) {
        self.ip.data_mut()[2..4].clone_from_slice(&s.to_be_bytes());
    }
    pub fn identifier(&self) -> u16 {
        u16::from_be_bytes(self.ip.data()[4..6].try_into().unwrap())
    }
    pub fn set_identifier(&mut self, i: u16) {
        self.ip.data_mut()[4..6].clone_from_slice(&i.to_be_bytes());
    }
    pub fn sequence_number(&self) -> u16 {
        u16::from_be_bytes(self.ip.data()[6..8].try_into().unwrap())
    }
    pub fn set_sequence_number(&mut self, n: u16) {
        self.ip.data_mut()[6..8].clone_from_slice(&n.to_be_bytes());
    }

    pub fn data(&self) -> &[u8] {
        &self.ip.data()[8..]
    }
    pub fn set_data(&mut self, data: &[u8]) {
        let data_len: u16 = data.len().try_into().unwrap();
        self.ip.set_size(8 + data_len);
        self.ip.data_mut()[8..].clone_from_slice(data);
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
