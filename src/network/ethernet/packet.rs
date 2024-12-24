extern crate alloc;

use alloc::boxed::Box;
use core::fmt;

use super::{MacAddress, Type};

pub struct Packet {
    pub(super) data: Box<[u8; 3000]>,
    size: usize,
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EthernetPacket")
            .field("destination", &self.mac_destination())
            .field("source", &self.mac_source())
            .field("ether_type", &self.ether_type())
            .finish()
    }
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            data: Box::new([0; 3000]),
            size: 0,
        }
    }

    pub fn mac_destination(&self) -> MacAddress {
        MacAddress(self.data[0..6].try_into().unwrap())
    }
    pub fn set_mac_destination(&mut self, d: MacAddress) {
        self.data[0..6].clone_from_slice(&d.0);
    }
    pub fn mac_source(&self) -> MacAddress {
        MacAddress(self.data[6..12].try_into().unwrap())
    }
    pub fn set_mac_source(&mut self, s: MacAddress) {
        self.data[6..12].clone_from_slice(&s.0);
    }
    pub fn ether_type(&self) -> Type {
        Type(u16::from_be_bytes(self.data[12..14].try_into().unwrap()))
    }
    pub fn set_ether_type(&mut self, t: Type) {
        self.data[12..14].clone_from_slice(&t.0.to_be_bytes());
    }

    pub fn size(&self) -> usize {
        self.size
    }
    pub fn header_size(&self) -> usize {
        14
    }
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }

    pub fn data(&mut self) -> &mut [u8] {
        let header_size = self.header_size();
        &mut self.data[header_size..header_size + self.size]
    }
}
