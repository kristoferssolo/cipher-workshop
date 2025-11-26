use std::fmt::Display;

use aes::{Aes, Block128};
use cipher_core::{BlockCipher, BlockError, CipherError};
use des::{Block64, Des};
use std::str::FromStr;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Des,
    Aes,
}

impl Algorithm {
    /// Creates a new cipher instance for the specified algorithm.
    ///
    /// Parses the key string and instantiates either DES or AES based on the algorithm choice.
    /// The key format depends on the algorithm:
    /// - DES: 64-bit key (hex string, e.g., "0x1334577999bcdff1")
    /// - AES: 128-bit key (hex string, e.g., "0x2b7e151628aed2a6abf7158809cf4f3c")
    ///
    /// # Returns
    ///
    /// A boxed cipher instance implementing `BlockCipher`, or a `CipherError` if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if the key cannot be parsed (invalid format, wrong length, etc.).
    ///
    pub fn new_cipher(&self, key: &str) -> Result<Box<dyn BlockCipher>, CipherError> {
        match self {
            Self::Des => {
                let key = Block64::from_str(key)?;
                let cipher = Des::from_key(key);
                Ok(Box::new(cipher))
            }
            Self::Aes => {
                let key = Block128::from_str(key)?;
                let cipher = Aes::from_key(key);
                Ok(Box::new(cipher))
            }
        }
    }

    /// Parses plaintext or ciphertext according to the specified algorithm's block size.
    ///
    /// Converts a text string into a byte vector using the appropriate block size:
    /// - DES: 64-bit blocks (8 bytes)
    /// - AES: 128-bit blocks (16 bytes)
    ///
    /// The input can be provided in various formats (hex, binary, ASCII, etc.) as supported
    /// by the block type's `FromStr` implementation.
    ///
    /// # Returns
    ///
    /// A byte vector representing the parsed text, or a `BlockError` if parsing fails.
    ///
    /// # Errors
    ///
    /// Returns `BlockError` if:
    /// - The text format is invalid
    /// - The text length doesn't match the block size
    /// - The text contains invalid characters for the given format
    ///
    pub fn parse_text(&self, text: &str) -> Result<Vec<u8>, BlockError> {
        match self {
            Self::Des => Ok(Block64::from_str(text)?.to_be_bytes().to_vec()),
            Self::Aes => Ok(Block128::from_str(text)?.to_be_bytes().to_vec()),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Des => "DES",
            Self::Aes => "AES",
        };
        f.write_str(s)
    }
}
