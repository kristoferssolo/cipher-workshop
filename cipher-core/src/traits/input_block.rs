use std::ops::{Deref, DerefMut};

/// Trait for fixed-size cipher block types.
///
/// Provides a common interface for accessing block data as byte slices,
/// used by cipher implementations to process input data.
///
/// # Implementers
/// - `Block64` (DES): 8-byte blocks
/// - `Block128` (AES): 16-byte blocks
pub trait InputBlock: Sized {
    /// Block size in bits (e.g., 64 for DES, 128 for AES).
    const BLOCK_SIZE: usize;

    /// Returns the block data as a byte slice.
    fn as_bytes(&self) -> &[u8];

    /// Returns the block data as a mutable byte slice.
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

/// Wrapper for parsing input blocks with automatic dereferencing.
#[derive(Debug, Clone)]
pub struct BlockParser<T: InputBlock>(pub T);

impl<T: InputBlock> Deref for BlockParser<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: InputBlock> DerefMut for BlockParser<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
