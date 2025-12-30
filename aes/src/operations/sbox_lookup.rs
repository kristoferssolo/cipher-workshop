use crate::{Block128, sbox::SboxLookup};

/// Substitutes each byte using the AES S-box (SubBytes step).
///
/// Provides non-linearity by replacing each byte with its S-box lookup value.
/// The S-box is derived from the multiplicative inverse in GF(2^8).
#[inline]
#[must_use]
pub fn sub_bytes(block: Block128) -> Block128 {
    block.sbox_lookup()
}

/// Inverse of [`sub_bytes`] using the inverse S-box.
#[inline]
#[must_use]
pub fn inv_sub_bytes(block: Block128) -> Block128 {
    block.inv_sbox_lookup()
}
