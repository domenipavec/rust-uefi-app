use core::fmt;
use uefi::proto::network::MacAddress as UefiMacAddress;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct MacAddress(pub [u8; 6]);

impl Into<UefiMacAddress> for MacAddress {
    fn into(self) -> UefiMacAddress {
        let mut result = UefiMacAddress([0; 32]);
        result.0[0..6].clone_from_slice(&self.0);
        result
    }
}

impl fmt::Debug for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "MacAddress({:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X})",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        ))
    }
}
