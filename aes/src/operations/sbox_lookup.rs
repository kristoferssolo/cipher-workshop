use crate::{Block128, sbox::SboxLookup};

#[inline]
#[must_use]
pub fn sub_bytes(block: Block128) -> Block128 {
    block.sbox_lookup()
}

#[inline]
#[must_use]
pub fn inv_sub_bytes(block: Block128) -> Block128 {
    block.inv_sbox_lookup()
}
