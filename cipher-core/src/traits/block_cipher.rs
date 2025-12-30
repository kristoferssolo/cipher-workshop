use crate::{CipherAction, CipherError, CipherResult, Output};

/// Generic block cipher trait for symmetric encryption algorithms.
///
/// Provides a standard encrypt/decrypt interface for block ciphers like AES and DES.
/// Implementers define [`transform_impl`](Self::transform_impl) to handle the core algorithm,
/// while [`transform`](Self::transform) provides block size validation.
///
/// # Example
/// ```ignore
/// use cipher_core::{BlockCipher, CipherAction};
///
/// let cipher = Aes::new(key);
/// let ciphertext = cipher.encrypt(&plaintext)?;
/// let decrypted = cipher.decrypt(&ciphertext)?;
/// ```
pub trait BlockCipher {
    /// Returns the block size in bytes for this cipher.
    ///
    /// Common values: 8 bytes (DES), 16 bytes (AES-128).
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
    /// Returns `CipherError::InvalidBlockSize` if the plaintext is not exactly `block_size()` bytes.
    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Output> {
        self.transform(plaintext, CipherAction::Encrypt)
    }

    /// Decrypts a single block.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if the ciphertext is not exactly `block_size()` bytes.
    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Output> {
        self.transform(ciphertext, CipherAction::Decrypt)
    }
}
