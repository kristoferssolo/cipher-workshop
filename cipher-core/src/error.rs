use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
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

impl CryptoError {
    /// Creates a key size error
    pub fn invalid_key_size(expected: usize, actual: usize) -> Self {
        CryptoError::InvalidKeySize { expected, actual }
    }

    /// Creates a block size error  
    pub fn invalid_block_size(expected: usize, actual: usize) -> Self {
        CryptoError::InvalidBlockSize { expected, actual }
    }

    /// Creates an invalid padding error
    pub fn invalid_padding() -> Self {
        CryptoError::InvalidPadding
    }

    /// Creates a plaintext length error
    pub fn invalid_plaintext_length(actual: usize) -> Self {
        CryptoError::InvalidPlaintextLength { actual }
    }

    /// Returns true if this is a key size error
    pub fn is_key_error(&self) -> bool {
        matches!(self, CryptoError::InvalidKeySize { .. })
    }

    /// Returns true if this is a block size error
    pub fn is_block_error(&self) -> bool {
        matches!(self, CryptoError::InvalidBlockSize { .. })
    }

    /// Returns true if this is a size-related error
    pub fn is_size_error(&self) -> bool {
        self.is_key_error()
            || self.is_block_error()
            || matches!(self, CryptoError::InvalidSize { .. })
    }

    /// Returns the expected size for size-related errors
    pub fn expected_size(&self) -> Option<usize> {
        match self {
            CryptoError::InvalidKeySize { expected, .. } => Some(*expected),
            CryptoError::InvalidBlockSize { expected, .. } => Some(*expected),
            CryptoError::InvalidSize { expected, .. } => Some(*expected),
            _ => None,
        }
    }

    /// Returns the actual size for size-related errors
    pub fn actual_size(&self) -> Option<usize> {
        match self {
            CryptoError::InvalidKeySize { actual, .. } => Some(*actual),
            CryptoError::InvalidBlockSize { actual, .. } => Some(*actual),
            CryptoError::InvalidSize { actual, .. } => Some(*actual),
            CryptoError::InvalidPlaintextLength { actual } => Some(*actual),
            _ => None,
        }
    }
}

/// Type alias for clean Result types
pub type CryptoResult<T> = core::result::Result<T, CryptoError>;
