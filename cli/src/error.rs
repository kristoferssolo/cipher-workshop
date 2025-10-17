use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("String contains no content")]
    EmptyString,

    #[error("File '{0}' contains no content")]
    EmptyFile(PathBuf),

    #[error("Failed to find file '{0}'. File does not exist")]
    MissingFile(PathBuf),

    #[error("Failed to read file '{0}'. Cannot read file contents")]
    FileReadingError(PathBuf),

    #[error("Invalid number format: {0}")]
    InvalidFormat(String),

    #[error("Invalid byte string length: expected no more than 8, found {0}")]
    InvalidByteStringLength(usize),

    #[error("String-to-u64 conversion error: {0}")]
    ConversionError(String),
}
