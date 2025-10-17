use crate::block::{block32::Block32, block64::Block64};
use std::mem::swap;

#[derive(Debug, Clone, Copy)]
pub struct LR {
    pub(crate) left: Block32,
    pub(crate) right: Block32,
}

impl LR {
    #[must_use]
    pub fn new(left: impl Into<Block32>, right: impl Into<Block32>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }

    #[inline]
    pub const fn swap(&mut self) {
        swap(&mut self.left, &mut self.right);
    }

    #[inline]
    #[must_use]
    pub const fn left(self) -> Block32 {
        self.left
    }

    #[inline]
    #[must_use]
    pub const fn right(self) -> Block32 {
        self.right
    }
}

impl From<LR> for Block64 {
    fn from(lr: LR) -> Self {
        let left = lr.left.as_u64() << 32;
        let right = lr.right.as_u64();
        Self::new(left | right)
    }
}
