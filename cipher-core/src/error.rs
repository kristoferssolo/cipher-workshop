use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum CipherError {
    /// Invalid key size for the cipher
    #[error("Invalid key size: expected {expected} bytes, got {actual}")]
    InvalidKeySize { expected: usize, actual: usize },

    /// Input data doesn't match the cipher's block size
    #[error("Invalid block size: expected {expected} bytes, got {actual}")]
    InvalidBlockSize { expected: usize, actual: usize },
}

impl CipherError {
    #[inline]
    #[must_use]
    pub const fn invalid_key_size(expected: usize, actual: usize) -> Self {
        Self::InvalidKeySize { expected, actual }
    }

    #[inline]
    #[must_use]
    pub const fn invalid_block_size(expected: usize, actual: usize) -> Self {
        Self::InvalidBlockSize { expected, actual }
    }
}

/// Type alias for clean Result types
pub type CipherResult<T> = core::result::Result<T, CipherError>;
