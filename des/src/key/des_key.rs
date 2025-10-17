use std::fmt::Debug;
use zeroize::ZeroizeOnDrop;

/// 64-bit Key for DES
#[derive(ZeroizeOnDrop)]
pub struct Key([u8; 8]);
impl Key {
    #[inline]
    #[must_use]
    pub const fn from_array(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[u8; 8] {
        &self.0
    }

    #[inline]
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }
}

impl From<[u8; 8]> for Key {
    fn from(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
}

impl From<Key> for [u8; 8] {
    fn from(key: Key) -> Self {
        key.0
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<u64> for Key {
    fn from(key: u64) -> Self {
        Self(key.to_be_bytes())
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key([REDACTED])")
    }
}
