use core::fmt;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Address(pub [u8; 4]);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "IpAddress({}.{}.{}.{})",
            self.0[0], self.0[1], self.0[2], self.0[3],
        ))
    }
}
