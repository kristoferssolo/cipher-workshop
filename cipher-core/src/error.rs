use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum CipherError {
    /// Invalid key size for the cipher
    #[error("invalid key size: expected {expected} bytes, got {actual}")]
    InvalidKeySize { expected: usize, actual: usize },

    /// Input data doesn't match the cipher's block size
    #[error("invalid block size: expected {expected} bytes, got {actual}")]
    InvalidBlockSize { expected: usize, actual: usize },

    /// Decryption detected invalid padding
    #[error("invalid padding detected during decryption")]
    InvalidPadding,

    /// Input length not valid for unpadded operation
    #[error("invalid plaintext length: {actual} bytes (must be multiple of block size)")]
    InvalidPlaintextLength { actual: usize },

    /// General size validation failure
    #[error("size mismatch: expected {expected} bytes, got {actual}")]
    InvalidSize { expected: usize, actual: usize },
}

impl CipherError {
    /// Creates a key size error
    #[inline]
    #[must_use]
    pub const fn invalid_key_size(expected: usize, actual: usize) -> Self {
        Self::InvalidKeySize { expected, actual }
    }

    /// Creates a block size error  
    #[inline]
    #[must_use]
    pub const fn invalid_block_size(expected: usize, actual: usize) -> Self {
        Self::InvalidBlockSize { expected, actual }
    }

    /// Creates an invalid padding error
    #[inline]
    #[must_use]
    pub const fn invalid_padding() -> Self {
        Self::InvalidPadding
    }

    /// Creates a plaintext length error
    #[inline]
    #[must_use]
    pub const fn invalid_plaintext_length(actual: usize) -> Self {
        Self::InvalidPlaintextLength { actual }
    }

    /// Returns true if this is a key size error
    #[inline]
    #[must_use]
    pub const fn is_key_error(&self) -> bool {
        matches!(self, Self::InvalidKeySize { .. })
    }

    /// Returns true if this is a block size error
    #[inline]
    #[must_use]
    pub const fn is_block_error(&self) -> bool {
        matches!(self, Self::InvalidBlockSize { .. })
    }

    /// Returns true if this is a size-related error
    #[must_use]
    pub const fn is_size_error(&self) -> bool {
        self.is_key_error() || self.is_block_error() || matches!(self, Self::InvalidSize { .. })
    }

    /// Returns the expected size for size-related errors
    #[must_use]
    pub const fn expected_size(&self) -> Option<usize> {
        match self {
            Self::InvalidKeySize { expected, .. }
            | Self::InvalidBlockSize { expected, .. }
            | Self::InvalidSize { expected, .. } => Some(*expected),
            _ => None,
        }
    }

    /// Returns the actual size for size-related errors
    #[must_use]
    pub const fn actual_size(&self) -> Option<usize> {
        match self {
            Self::InvalidKeySize { actual, .. }
            | Self::InvalidBlockSize { actual, .. }
            | Self::InvalidSize { actual, .. }
            | Self::InvalidPlaintextLength { actual } => Some(*actual),
            Self::InvalidPadding => None,
        }
    }
}

/// Type alias for clean Result types
pub type CipherResult<T> = core::result::Result<T, CipherError>;
