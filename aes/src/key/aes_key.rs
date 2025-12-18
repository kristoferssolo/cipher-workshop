use crate::Block128;
use std::fmt::Debug;
use zeroize::ZeroizeOnDrop;

/// 128-bit Key for AES
#[derive(ZeroizeOnDrop)]
pub struct Key([u8; 16]);

impl Key {
    #[inline]
    #[must_use]
    pub const fn from_array(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[u8; 16] {
        &self.0
    }

    #[inline]
    #[must_use]
    pub const fn as_2d(&self) -> [[u8; 4]; 4] {
        [
            [self.0[0], self.0[1], self.0[2], self.0[3]],
            [self.0[4], self.0[5], self.0[6], self.0[7]],
            [self.0[8], self.0[9], self.0[10], self.0[11]],
            [self.0[12], self.0[13], self.0[14], self.0[15]],
        ]
    }

    #[inline]
    #[must_use]
    pub const fn as_u128(&self) -> u128 {
        u128::from_be_bytes(self.0)
    }
}

impl From<[[u8; 4]; 4]> for Key {
    fn from(matrix: [[u8; 4]; 4]) -> Self {
        let mut bytes = [0; 16];
        for (idx, row) in matrix.iter().enumerate() {
            bytes[idx * 4..(idx + 1) * 4].copy_from_slice(row);
        }
        Self(bytes)
    }
}

impl From<[u8; 16]> for Key {
    fn from(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl From<&[u8]> for Key {
    fn from(value: &[u8]) -> Self {
        let mut bytes = [0; 16];
        let len = value.len().min(16);
        bytes[..len].copy_from_slice(&value[..len]);
        bytes.into()
    }
}

impl From<u128> for Key {
    fn from(key: u128) -> Self {
        key.to_be_bytes().into()
    }
}

impl From<Block128> for Key {
    fn from(key: Block128) -> Self {
        key.to_be_bytes().into()
    }
}

impl From<Key> for [u8; 16] {
    fn from(key: Key) -> Self {
        key.0
    }
}

impl From<Key> for [[u8; 4]; 4] {
    fn from(key: Key) -> Self {
        key.as_2d()
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key([REDACTED])")
    }
}
