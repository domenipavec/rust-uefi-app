use core::fmt;

use uefi_raw::newtype_enum;

use crate::network::{ethernet, ip};

pub struct Packet<'p>(pub &'p mut [u8]);

impl fmt::Debug for Packet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArpPacket")
            .field("hardware_type", &self.hardware_type())
            .field("protocol_type", &self.protocol_type())
            .field("hardware_len", &self.hardware_len())
            .field("protocol_len", &self.protocol_len())
            .field("operation", &self.operation())
            .field("sender_hardware_address", &self.sender_hardware_address())
            .field("sender_protocol_address", &self.sender_protocol_address())
            .field("target_hardware_address", &self.target_hardware_address())
            .field("target_protocol_address", &self.target_protocol_address())
            .finish()
    }
}

impl Packet<'_> {
    pub fn hardware_type(&self) -> HardwareType {
        HardwareType(u16::from_be_bytes(self.0[0..2].try_into().unwrap()))
    }
    pub fn set_hardware_type(&mut self, ht: HardwareType) {
        self.0[0..2].clone_from_slice(&ht.0.to_be_bytes());
    }
    pub fn protocol_type(&self) -> ethernet::Type {
        ethernet::Type(u16::from_be_bytes(self.0[2..4].try_into().unwrap()))
    }
    pub fn set_protocol_type(&mut self, pt: ethernet::Type) {
        self.0[2..4].clone_from_slice(&pt.0.to_be_bytes());
    }
    pub fn hardware_len(&self) -> u8 {
        self.0[4]
    }
    pub fn set_hardware_len(&mut self, l: u8) {
        self.0[4] = l;
    }
    pub fn protocol_len(&self) -> u8 {
        self.0[5]
    }
    pub fn set_protocol_len(&mut self, l: u8) {
        self.0[5] = l;
    }
    pub fn operation(&self) -> Operation {
        Operation(u16::from_be_bytes(self.0[6..8].try_into().unwrap()))
    }
    pub fn set_operation(&mut self, o: Operation) {
        self.0[6..8].clone_from_slice(&o.0.to_be_bytes());
    }
    pub fn sender_hardware_address(&self) -> ethernet::MacAddress {
        ethernet::MacAddress(self.0[8..14].try_into().unwrap())
    }
    pub fn set_sender_hardware_address(&mut self, a: &ethernet::MacAddress) {
        self.0[8..14].clone_from_slice(&a.0);
    }
    pub fn sender_protocol_address(&self) -> ip::Address {
        ip::Address(self.0[14..18].try_into().unwrap())
    }
    pub fn set_sender_protocol_address(&mut self, a: &ip::Address) {
        self.0[14..18].clone_from_slice(&a.0);
    }
    pub fn target_hardware_address(&self) -> ethernet::MacAddress {
        ethernet::MacAddress(self.0[18..24].try_into().unwrap())
    }
    pub fn set_target_hardware_address(&mut self, a: &ethernet::MacAddress) {
        self.0[18..24].clone_from_slice(&a.0);
    }
    pub fn target_protocol_address(&self) -> ip::Address {
        ip::Address(self.0[24..28].try_into().unwrap())
    }
    pub fn set_target_protocol_address(&mut self, a: &ip::Address) {
        self.0[24..28].clone_from_slice(&a.0);
    }
}

newtype_enum! {
    pub enum Operation: u16 => {
        REQUEST = 1,
        RESPONSE = 2,
    }
}

newtype_enum! {
    pub enum HardwareType: u16 => {
        ETHERNET = 1,
    }
}
