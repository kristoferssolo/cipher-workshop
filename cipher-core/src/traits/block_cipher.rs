use crate::{CipherAction, CipherError, CipherResult, Output};

/// Generic block cipher trait.
///
/// Implements the standard encrypt/decrypt interface for block ciphers.
/// Implementers define `transform_impl` to handle the core algorithm,
/// while `transform` provides validation and convenience wrappers.
pub trait BlockCipher {
    fn block_size(&self) -> usize;

    /// Core cipher transformation (must be implemented by concrete types).
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if the transformation fails.
    fn transform_impl(&self, block: &[u8], action: CipherAction) -> CipherResult<Output>;

    /// Transforms a block with validation.
    ///
    /// Validates that the block size matches `BLOCK_SIZE` before delegating
    /// to `transform_impl`.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if `block.len() != self.block_size()`.
    fn transform(&self, block: &[u8], action: CipherAction) -> CipherResult<Output> {
        let block_size = self.block_size();
        if block.len() != block_size {
            return Err(CipherError::invalid_block_size(block_size, block.len()));
        }
        self.transform_impl(block, action)
    }

    /// Encrypts a single block.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if the plaintext is not exactly `BLOCK_SIZE` bytes.
    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Output> {
        self.transform(plaintext, CipherAction::Encrypt)
    }

    /// Decrypts a single block.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if the plaintext is not exactly `BLOCK_SIZE` bytes.
    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Output> {
        self.transform(ciphertext, CipherAction::Decrypt)
    }
}
