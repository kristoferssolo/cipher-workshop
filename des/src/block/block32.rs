use std::ops::BitXor;

use super::secret_block;

secret_block! {
    pub struct Block32(u32, 32, 0xFFFF_FFFF);
}

impl From<u64> for Block32 {
    fn from(value: u64) -> Self {
        let mask = u64::from(Self::MASK);
        let value = u32::try_from(value & mask).unwrap_or_default();
        Self(value)
    }
}

impl BitXor for Block32 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
