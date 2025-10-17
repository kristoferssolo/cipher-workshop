use crate::block::lr::LR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block64(u64);

impl Block64 {
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    #[inline]
    #[must_use]
    pub fn split_lr(self) -> LR {
        self.into()
    }
}

impl From<[u8; 8]> for Block64 {
    fn from(bytes: [u8; 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl From<u64> for Block64 {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Block64> for LR {
    fn from(block: Block64) -> Self {
        let left = (block.0 >> 32) as u32;
        let right = (block.0 & 0xFFFF_FFFF) as u32;
        Self::new(left, right)
    }
}

impl From<&Block64> for LR {
    fn from(block: &Block64) -> Self {
        let left = (block.0 >> 32) as u32;
        let right = (block.0 & 0xFFFF_FFFF) as u32;
        Self::new(left, right)
    }
}

impl From<Block64> for Vec<u8> {
    fn from(value: Block64) -> Self {
        value.0.to_be_bytes().to_vec()
    }
}

impl From<&Block64> for Vec<u8> {
    fn from(value: &Block64) -> Self {
        value.0.to_be_bytes().to_vec()
    }
}
