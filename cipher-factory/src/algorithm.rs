use std::fmt::Display;

use aes::{Aes, AesCbc, Block128, Iv};
use cipher_core::{BlockCipher, BlockError, CipherError, CipherResult};
use des::{Block64, Des};
use std::str::FromStr;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Des,
    Aes,
    #[cfg_attr(feature = "clap", clap(name = "aes-cbc"))]
    AesCbc,
}

impl Algorithm {
    /// Returns whether this algorithm requires an IV (Initialization Vector).
    #[must_use]
    pub const fn requires_iv(&self) -> bool {
        matches!(self, Self::AesCbc)
    }

    /// Creates a new ECB-mode cipher instance for the specified algorithm.
    ///
    /// Parses the key string and instantiates either DES or AES based on the algorithm choice.
    /// The key format depends on the algorithm:
    /// - DES: 64-bit key (hex string, e.g., "0x1334577999bcdff1")
    /// - AES: 128-bit key (hex string, e.g., "0x2b7e151628aed2a6abf7158809cf4f3c")
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if the key cannot be parsed or if called with a CBC algorithm.
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
            Self::AesCbc => Err(CipherError::InvalidPadding(
                "AES-CBC requires an IV; use new_cbc_cipher instead".into(),
            )),
        }
    }

    /// Creates a new AES-CBC cipher instance with the given key and IV.
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if the key or IV cannot be parsed.
    pub fn new_cbc_cipher(&self, key: &str, iv: &str) -> Result<AesCbc, CipherError> {
        match self {
            Self::AesCbc => {
                let key = Block128::from_str(key)?;
                let iv = Iv::from_str(iv)?;
                Ok(AesCbc::new(key, iv))
            }
            _ => Err(CipherError::InvalidPadding(format!(
                "{self} does not support CBC mode"
            ))),
        }
    }

    /// Encrypts data using CBC mode with PKCS#7 padding.
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if encryption fails.
    pub fn encrypt_cbc(&self, key: &str, iv: &str, plaintext: &[u8]) -> CipherResult<Vec<u8>> {
        let cipher = self.new_cbc_cipher(key, iv)?;
        cipher.encrypt(plaintext)
    }

    /// Decrypts data using CBC mode and removes PKCS#7 padding.
    ///
    /// The IV is extracted from the first 16 bytes of the ciphertext.
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if decryption fails or padding is invalid.
    pub fn decrypt_cbc(&self, key: &str, ciphertext: &[u8]) -> CipherResult<Vec<u8>> {
        // IV is embedded in ciphertext, use dummy IV for cipher construction
        let dummy_iv = "0x00000000000000000000000000000000";
        let cipher = self.new_cbc_cipher(key, dummy_iv)?;
        cipher.decrypt(ciphertext)
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
    /// Parses text for ECB-mode algorithms (single block).
    ///
    /// For CBC mode, use raw bytes directly instead of this method.
    pub fn parse_text(&self, text: &str) -> Result<Vec<u8>, BlockError> {
        match self {
            Self::Des => Ok(Block64::from_str(text)?.to_be_bytes().to_vec()),
            Self::Aes | Self::AesCbc => Ok(Block128::from_str(text)?.to_be_bytes().to_vec()),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Des => "DES",
            Self::Aes => "AES",
            Self::AesCbc => "AES-CBC",
        };
        f.write_str(s)
    }
}
