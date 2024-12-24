use uefi_raw::newtype_enum;

newtype_enum! {
    pub enum Type: u16 => {
        IPV4 = 0x0800,
        ARP = 0x0806,
        WOL = 0x0842,
    }
}
