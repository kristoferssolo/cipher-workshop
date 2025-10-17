use crate::{
    key::{cd56::CD56, half28::Half28},
    secret_int,
};

secret_int! {
    /// 56-bit key after PC-1 (lower 56 bits used).
    pub struct Key56(u64, 56, 0x00FF_FFFF_FFFF_FFFF);
}

impl Key56 {
    #[must_use]
    pub fn split(&self) -> CD56 {
        let c = ((self.0 >> 28) & 0x0FFF_FFFF) as u32;
        let d = (self.0 & 0x0FFF_FFFF) as u32;
        CD56::new(c, d)
    }

    #[must_use]
    pub fn from_half28(left: &Half28, right: &Half28) -> Self {
        let left = u64::from(left.as_int());
        let right = u64::from(right.as_int());
        Self::from_int((left << 28) | right)
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
