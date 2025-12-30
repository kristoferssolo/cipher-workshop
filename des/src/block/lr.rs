use crate::block::{block32::Block32, block64::Block64};
use std::mem::swap;

/// Left-Right pair representing DES state during Feistel rounds.
///
/// DES splits a 64-bit block into two 32-bit halves (L and R) for
/// processing through 16 Feistel rounds.
#[derive(Debug, Clone, Copy)]
pub struct LR {
    pub(crate) left: Block32,
    pub(crate) right: Block32,
}

impl LR {
    /// Creates a new L-R pair from two 32-bit blocks.
    #[must_use]
    pub fn new(left: impl Into<Block32>, right: impl Into<Block32>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }

    /// Swaps the left and right halves in place.
    #[inline]
    pub const fn swap(&mut self) {
        swap(&mut self.left, &mut self.right);
    }

    /// Returns the left 32-bit half.
    #[inline]
    #[must_use]
    pub const fn left(self) -> Block32 {
        self.left
    }

    /// Returns the right 32-bit half.
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
