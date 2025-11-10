use crate::key::{expanded::ExpandedKey, secret_key};
use std::ops::BitXor;

secret_key! {
    /// A single AES round subkey
    pub struct Subkey(u32, 32, 0xFFFF_FFFF);
}

impl Subkey {
    /// Zero value.
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Shifts the bits to the left by a specified amount, `n`,
    /// wrapping the truncated bits to the end of the resulting integer.
    ///
    /// Please note this isn't the same operation as the `<<` shifting operator!
    pub const fn rotate_left(self, n: u32) -> Self {
        Self(self.0.rotate_left(n))
    }

    /// Shifts the bits to the right by a specified amount, `n`,
    /// wrapping the truncated bits to the beginning of the resulting integer.
    ///
    /// Please note this isn't the same operation as the `>>` shifting operator!
    pub const fn rotate_right(self, n: u32) -> Self {
        Self(self.0.rotate_right(n))
    }
}

impl BitXor<ExpandedKey> for Subkey {
    type Output = Self;
    fn bitxor(self, rhs: ExpandedKey) -> Self::Output {
        Self(self.0 ^ rhs.as_u32())
    }
}

impl BitXor for Subkey {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
