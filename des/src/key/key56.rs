use crate::{
    key::{cd56::CD56, half28::Half28},
    secret_key,
};

secret_key! {
    /// 56-bit key after PC-1 (lower 56 bits used).
    pub struct Key56(u64, 56, 0x00FF_FFFF_FFFF_FFFF);
}

impl Key56 {
    #[must_use]
    pub const fn split(&self) -> CD56 {
        let c = ((self.0 >> 28) & 0x0FFF_FFFF) as u32;
        let d = (self.0 & 0x0FFF_FFFF) as u32;
        CD56::new(Half28::from_u32(c), Half28::from_u32(d))
    }

    #[must_use]
    pub const fn from_half28(left: &Half28, right: &Half28) -> Self {
        let left = left.as_u64();
        let right = right.as_u64();
        Self::from_u64((left << 28) | right)
    }
}

impl From<Key56> for CD56 {
    fn from(key56: Key56) -> Self {
        key56.split()
    }
}

impl From<&Key56> for CD56 {
    fn from(key56: &Key56) -> Self {
        key56.split()
    }
}
