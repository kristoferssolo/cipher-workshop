//! Initialization Vector (IV) for AES block cipher modes.
//!
//! The IV provides randomness to ensure identical plaintexts produce
//! different ciphertexts. For CBC mode, the IV must be unpredictable
//! and should never be reused with the same key.

use crate::Block128;
use cipher_core::{BlockError, parse_block_int, secret_block};
use std::{fmt, str::FromStr};

secret_block! {
    /// 128-bit Initialization Vector for AES cipher modes.
    ///
    /// Used in CBC mode to XOR with the first plaintext block before encryption.
    /// Each subsequent block is XORed with the previous ciphertext block.
    ///
    /// # Security
    ///
    /// - IVs must be unpredictable (use a cryptographically secure RNG)
    /// - Never reuse an IV with the same key
    /// - The IV does not need to be secret, but must be unique
    pub struct Iv(u128, 128, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
}

impl Iv {
    /// Creates an IV from big-endian bytes.
    #[inline]
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    /// Returns the IV as big-endian bytes.
    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Converts this IV to a Block128 for XOR operations.
    #[inline]
    #[must_use]
    pub const fn to_block(self) -> Block128 {
        Block128::new(self.0)
    }
}

impl fmt::Display for Iv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032X}", self.0)
    }
}

impl FromStr for Iv {
    type Err = BlockError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_block_int::<u128>(s)?))
    }
}

impl From<[u8; 16]> for Iv {
    fn from(bytes: [u8; 16]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl From<Block128> for Iv {
    fn from(block: Block128) -> Self {
        Self(block.as_u128())
    }
}

impl From<Iv> for Block128 {
    fn from(iv: Iv) -> Self {
        Self::new(iv.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_ok;

    #[test]
    fn iv_from_hex_string() {
        let iv = assert_ok!("0x000102030405060708090A0B0C0D0E0F".parse::<Iv>());
        assert_eq!(iv.as_u128(), 0x0001_0203_0405_0607_0809_0A0B_0C0D_0E0F);
    }

    #[test]
    fn iv_to_block_conversion() {
        let iv = Iv::new(0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF);
        let block = Block128::from(iv);
        assert_eq!(block.as_u128(), 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF);
    }

    #[test]
    fn iv_from_bytes() {
        let bytes = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ];
        let iv = Iv::from_be_bytes(bytes);
        assert_eq!(iv.as_u128(), 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF);
    }

    #[test]
    fn iv_display_format() {
        let iv = Iv::new(0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF);
        assert_eq!(format!("{iv}"), "00112233445566778899AABBCCDDEEFF");
        assert_eq!(format!("{iv:x}"), "00112233445566778899aabbccddeeff");
    }
}
