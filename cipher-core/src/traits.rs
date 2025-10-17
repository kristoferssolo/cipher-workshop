use crate::{CipherAction, CipherError, CipherOutput, CipherResult};

/// Generic block cipher trait.
///
/// Implements the standard encrypt/decrypt interface for block ciphers.
/// Implementers define `transform_impl` to handle the core algorithm,
/// while `transform` provides validation and convenience wrappers.
pub trait BlockCipher: Sized {
    const BLOCK_SIZE: usize;

    fn from_key(key: &[u8]) -> Self;

    /// Core cipher transformation (must be implemented by concrete types).
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if the transformation fails.
    fn transform_impl(&self, block: &[u8], action: CipherAction) -> CipherResult<CipherOutput>;

    /// Transforms a block with validation.
    ///
    /// Validates that the block size matches `BLOCK_SIZE` before delegating
    /// to `transform_impl`.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if `block.len() != BLOCK_SIZE`.
    fn transform(&self, block: &[u8], action: CipherAction) -> CipherResult<CipherOutput> {
        if block.len() != Self::BLOCK_SIZE {
            return Err(CipherError::invalid_block_size(
                Self::BLOCK_SIZE,
                block.len(),
            ));
        }
        self.transform_impl(block, action)
    }

    /// Encrypts a single block.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if the plaintext is not exactly `BLOCK_SIZE` bytes.
    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<CipherOutput> {
        self.transform(plaintext, CipherAction::Encrypt)
    }

    /// Decrypts a single block.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if the plaintext is not exactly `BLOCK_SIZE` bytes.
    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<CipherOutput> {
        self.transform(ciphertext, CipherAction::Decrypt)
    }
}
