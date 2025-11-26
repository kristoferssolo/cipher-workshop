use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CipherError {
    /// Invalid key size for the cipher
    #[error("Invalid key size: expected {expected} bytes, got {actual}.")]
    InvalidKeySize { expected: usize, actual: usize },

    /// Input data doesn't match the cipher's block size
    #[error("Invalid block size: expected {expected} bytes, got {actual}.")]
    InvalidBlockSize { expected: usize, actual: usize },

    /// Error parsing block from string
    #[error("{0}")]
    BlockParseError(#[from] BlockError),
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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// Input data is empty
    #[error("Inputed block is empty")]
    EmptyBlock,

    /// Parse error
    #[error("Error parsing block")]
    ParseError(#[from] ParseIntError),

    /// Byte size length
    #[error("Invalid byte string length: expected no more than 8, found {0}")]
    InvalidByteStringLength(usize),

    /// String to int conversion error
    #[error("String-to-{typ} conversion error: {err}")]
    ConversionError { typ: String, err: String },
}

impl BlockError {
    #[inline]
    #[must_use]
    pub fn conversion_error(typ: &str, err: &str) -> Self {
        Self::ConversionError {
            typ: typ.into(),
            err: err.into(),
        }
    }
}
