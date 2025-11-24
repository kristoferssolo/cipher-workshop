use crate::key::secret_key;
use std::ops::{BitOr, BitXor, Shl};

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

    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl BitXor for Subkey {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.as_u32())
    }
}

impl Shl<i32> for Subkey {
    type Output = u128;
    fn shl(self, rhs: i32) -> Self::Output {
        self.as_u128() << rhs
    }
}

impl BitOr<Subkey> for u128 {
    type Output = Self;
    fn bitor(self, rhs: Subkey) -> Self::Output {
        self | rhs.as_u128()
    }
}
