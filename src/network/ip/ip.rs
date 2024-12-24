use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
};
use log::info;
use uefi_raw::newtype_enum;

use crate::{
    arp, icmp,
    network::{self, EtherType, MacAddress},
};

pub struct Handler<'a> {
    pub ip: Address,
    pub arp: &'a mut arp::Service<'a>,
    pub sender: network::Sender<'a>,
}

impl Handler<'_> {
    pub fn handle(&mut self, data: &mut [u8], src: MacAddress) {
        let mut packet = Packet(data);
        if compute_checksum(&packet.0[..packet.header_len() as usize]) != 0 {
            return;
        }
        self.arp.set(packet.source_address(), src);

        let mut icmp_handler = icmp::Handler {};
        match packet.protocol() {
            Protocol::ICMP => icmp_handler.handle(self, &mut packet),
            _ => info!("{:?}", packet),
        }
        drop(icmp_handler);
    }

    pub fn send<F>(&mut self, dst: Address, protocol: Protocol, bf: F) -> Result<(), uefi::Error>
    where
        F: Fn(&mut [u8]) -> u16,
    {
        self.sender.send(self.arp.get(dst), EtherType::IPV4, |buf| {
            let mut packet = Packet(buf);
            packet.set_version(4);
            packet.set_header_len(20);
            let data_len = bf(packet.data());
            packet.set_total_len(packet.header_len() as u16 + data_len);
            packet.set_identification(0);
            packet.set_ttl(255);
            packet.set_protocol(protocol);
            packet.set_source_address(&self.ip);
            packet.set_destination_address(&dst);

            packet.set_header_checksum(compute_checksum(&packet.0[..packet.header_len() as usize]));

            packet.total_len()
        })
    }
}

pub struct Packet<'p>(pub &'p mut [u8]);

impl fmt::Debug for Packet<'_> {
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

impl Packet<'_> {
    pub fn version(&self) -> u8 {
        return (self.0[0] & 0xf0) >> 4;
    }
    fn set_version(&mut self, v: u8) {
        self.0[0] = (self.0[0] & 0xf) | (v << 4);
    }
    pub fn header_len(&self) -> u8 {
        return (self.0[0] & 0xf) * 4;
    }
    fn set_header_len(&mut self, l: u8) {
        self.0[0] = (self.0[0] & 0xf0) | ((l / 4) & 0xf);
    }
    pub fn total_len(&self) -> u16 {
        u16::from_be_bytes(self.0[2..4].try_into().unwrap())
    }
    fn set_total_len(&mut self, l: u16) {
        self.0[2..4].clone_from_slice(&l.to_be_bytes());
    }
    pub fn identification(&self) -> u16 {
        u16::from_be_bytes(self.0[4..6].try_into().unwrap())
    }
    fn set_identification(&mut self, l: u16) {
        self.0[4..6].clone_from_slice(&l.to_be_bytes());
    }
    pub fn ttl(&self) -> u8 {
        return self.0[8];
    }
    fn set_ttl(&mut self, ttl: u8) {
        self.0[8] = ttl
    }
    pub fn protocol(&self) -> Protocol {
        return Protocol(self.0[9]);
    }
    fn set_protocol(&mut self, p: Protocol) {
        self.0[9] = p.0
    }
    pub fn header_checksum(&self) -> u16 {
        u16::from_be_bytes(self.0[10..12].try_into().unwrap())
    }
    fn set_header_checksum(&mut self, s: u16) {
        self.0[10..12].clone_from_slice(&s.to_be_bytes());
    }
    pub fn source_address(&self) -> Address {
        Address(self.0[12..16].try_into().unwrap())
    }
    fn set_source_address(&mut self, a: &Address) {
        self.0[12..16].clone_from_slice(&a.0);
    }
    pub fn destination_address(&self) -> Address {
        Address(self.0[16..20].try_into().unwrap())
    }
    fn set_destination_address(&mut self, a: &Address) {
        self.0[16..20].clone_from_slice(&a.0);
    }
    pub fn data(&mut self) -> &mut [u8] {
        let header_len = self.header_len() as usize;

        let mut total_len = self.total_len() as usize;
        if total_len == 0 {
            total_len = self.0.len()
        }
        &mut self.0[header_len..total_len]
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

pub fn compute_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for i in 0..data.len() / 2 {
        sum += u16::from_be_bytes(data[2 * i..2 * i + 2].try_into().unwrap()) as u32;
    }
    if data.len() % 2 == 1 {
        sum += (data[data.len() - 1] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum >> 16) + (sum & 0xffff);
    }
    sum = !sum;
    sum as u16
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_checksum() {
        let data: [u8; 64] = [
            8, 0, 193, 180, 0, 18, 3, 33, 148, 17, 74, 103, 0, 0, 0, 0, 138, 204, 11, 0, 0, 0, 0,
            0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
            37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
        ];
        let result = super::compute_checksum(&data);
        assert_eq!(64, data.len());
        assert_eq!(0, result);
    }
}
