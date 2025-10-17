#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block6(u8);

impl Block6 {
    const MASK: u8 = 0x3F;

    #[inline]
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self(value & Self::MASK)
    }

    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn to_row(self) -> usize {
        ((self.0 >> 5) << 1 | (self.0 & 1)) as usize
    }

    #[inline]
    #[must_use]
    pub const fn to_col(self) -> usize {
        ((self.0 >> 1) & 0xF) as usize
    }
}
