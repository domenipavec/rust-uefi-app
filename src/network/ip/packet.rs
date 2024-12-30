use core::fmt;

use uefi_raw::newtype_enum;

use crate::network::ethernet;

use super::Address;

pub struct Packet {
    pub eth: ethernet::Packet,
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IPPacket")
            .field("version", &self.version())
            .field("header_len", &self.header_len())
            .field("total_len", &self.total_len())
            .field("identification", &self.identification())
            .field("ttl", &self.ttl())
            .field("protocol", &self.protocol())
            .field("header_checksum", &self.header_checksum())
            .field("source_address", &self.source_address())
            .field("destination_address", &self.destination_address())
            .finish()
    }
}

impl Packet {
    pub fn new() -> Packet {
        let mut p = Packet {
            eth: ethernet::Packet::new(),
        };

        p.set_header_len(20);
        p.set_version(4);
        p.set_identification(0);
        p.set_ttl(255);

        p.eth.set_ether_type(ethernet::Type::IPV4);

        p
    }

    pub fn version(&self) -> u8 {
        return (self.eth.data()[0] & 0xf0) >> 4;
    }
    pub fn set_version(&mut self, v: u8) {
        self.eth.data_mut()[0] = (self.eth.data()[0] & 0xf) | (v << 4);
    }
    pub fn header_len(&self) -> u8 {
        return (self.eth.data()[0] & 0xf) * 4;
    }
    pub fn set_header_len(&mut self, l: u8) {
        let lsize: usize = l.try_into().unwrap();
        if self.eth.size() < lsize {
            self.eth.set_size(lsize);
        }
        self.eth.data_mut()[0] = (self.eth.data()[0] & 0xf0) | ((l / 4) & 0xf);
    }
    pub fn total_len(&self) -> u16 {
        u16::from_be_bytes(self.eth.data()[2..4].try_into().unwrap())
    }
    pub fn set_total_len(&mut self, l: u16) {
        self.eth.set_size(l.try_into().unwrap());
        self.eth.data_mut()[2..4].clone_from_slice(&l.to_be_bytes());
    }
    pub fn identification(&self) -> u16 {
        u16::from_be_bytes(self.eth.data()[4..6].try_into().unwrap())
    }
    pub fn set_identification(&mut self, l: u16) {
        self.eth.data_mut()[4..6].clone_from_slice(&l.to_be_bytes());
    }
    pub fn ttl(&self) -> u8 {
        return self.eth.data()[8];
    }
    pub fn set_ttl(&mut self, ttl: u8) {
        self.eth.data_mut()[8] = ttl
    }
    pub fn protocol(&self) -> Protocol {
        return Protocol(self.eth.data()[9]);
    }
    pub fn set_protocol(&mut self, p: Protocol) {
        self.eth.data_mut()[9] = p.0
    }
    pub fn header_checksum(&self) -> u16 {
        u16::from_be_bytes(self.eth.data()[10..12].try_into().unwrap())
    }
    pub fn set_header_checksum(&mut self, s: u16) {
        self.eth.data_mut()[10..12].clone_from_slice(&s.to_be_bytes());
    }
    pub fn source_address(&self) -> Address {
        Address(self.eth.data()[12..16].try_into().unwrap())
    }
    pub fn set_source_address(&mut self, a: &Address) {
        self.eth.data_mut()[12..16].clone_from_slice(&a.0);
    }
    pub fn destination_address(&self) -> Address {
        Address(self.eth.data()[16..20].try_into().unwrap())
    }
    pub fn set_destination_address(&mut self, a: &Address) {
        self.eth.data_mut()[16..20].clone_from_slice(&a.0);
    }

    pub fn size(&self) -> u16 {
        let header_len: u16 = self.header_len().try_into().unwrap();
        self.total_len() - header_len
    }
    pub fn set_size(&mut self, size: u16) {
        let header_len: u16 = self.header_len().try_into().unwrap();
        self.set_total_len(size + header_len);
    }
    pub fn header(&self) -> &[u8] {
        let header_len = self.header_len() as usize;

        &self.eth.data()[..header_len]
    }

    pub fn data(&self) -> &[u8] {
        let header_len = self.header_len() as usize;

        let mut total_len = self.total_len() as usize;
        if total_len == 0 {
            total_len = self.eth.data().len()
        }
        &self.eth.data()[header_len..total_len]
    }
    pub fn data_mut(&mut self) -> &mut [u8] {
        let header_len = self.header_len() as usize;

        let mut total_len = self.total_len() as usize;
        if total_len == 0 {
            total_len = self.eth.data().len()
        }
        &mut self.eth.data_mut()[header_len..total_len]
    }
}

newtype_enum! {
    pub enum Protocol: u8 => {
        ICMP = 1,
        IGMP = 2,
        TCP = 6,
        UDP = 17,
    }
}
