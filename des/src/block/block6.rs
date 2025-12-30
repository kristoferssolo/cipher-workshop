use super::secret_block;

secret_block! {
    pub struct Block6(u8, 6, 0x3F);
}

impl Block6 {
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
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
