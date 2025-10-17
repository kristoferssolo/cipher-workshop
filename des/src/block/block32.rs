use std::ops::BitXor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block32(u32);

impl Block32 {
    const MASK: u32 = 0xFFFF_FFFF;

    #[inline]
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0 as u64
    }
}

impl From<u32> for Block32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
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
