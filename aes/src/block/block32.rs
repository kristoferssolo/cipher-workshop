use std::ops::BitXor;

use crate::{block::secret_block, key::Subkey};

secret_block! {
    pub struct Block32(u32, 32, 0xFFFF_FFFF);
}
impl Block32 {
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

impl BitXor<Subkey> for Block32 {
    type Output = Self;
    fn bitxor(self, rhs: Subkey) -> Self::Output {
        Self(self.0 ^ rhs.as_u32())
    }
}
