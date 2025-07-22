use core::{fmt, ops::BitAnd};

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

impl BitAnd for Address {
    type Output = Address;

    fn bitand(self, rhs: Address) -> Self::Output {
        let mut result = Address([0; 4]);
        for (i, _) in self.0.iter().enumerate() {
            result.0[i] = self.0[i] & rhs.0[i];
        }
        result
    }
}
