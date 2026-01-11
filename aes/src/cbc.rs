//! AES-CBC (Cipher Block Chaining) mode implementation.
//!
//! CBC mode combines each plaintext block with the previous ciphertext block
//! (using XOR) before encryption. The first block uses an Initialization Vector (IV).

use crate::{Aes, Block128, Iv, key::Key};
use cipher_core::{CipherError, CipherResult, pkcs7_pad, pkcs7_unpad};

const BLOCK_SIZE: usize = 16;

/// AES cipher in CBC (Cipher Block Chaining) mode.
///
/// CBC mode provides semantic security by combining each plaintext block
/// with the previous ciphertext block (via XOR) before encryption.
///
/// # Example
///
/// ```
/// use aes::{AesCbc, Iv};
///
/// let key = 0x2b7e1516_28aed2a6_abf71588_09cf4f3c_u128;
/// let iv = Iv::new(0x00010203_04050607_08090a0b_0c0d0e0f_u128);
/// let cipher = AesCbc::new(key, iv);
///
/// let plaintext = b"Hello, World!";
/// let ciphertext = cipher.encrypt(plaintext).unwrap();
/// let decrypted = cipher.decrypt(&ciphertext).unwrap();
/// assert_eq!(decrypted, plaintext);
/// ```
pub struct AesCbc {
    aes: Aes,
    iv: Iv,
}

impl AesCbc {
    /// Creates a new AES-CBC cipher with the given key and IV.
    #[must_use]
    pub fn new(key: impl Into<Key>, iv: impl Into<Iv>) -> Self {
        Self {
            aes: Aes::from_key(key),
            iv: iv.into(),
        }
    }

    /// Encrypts plaintext using CBC mode with PKCS#7 padding.
    ///
    /// The output format is: `[16-byte IV][ciphertext...]`
    ///
    /// # Errors
    ///
    /// Returns `CipherError` if encryption fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Vec<u8>> {
        let padded = pkcs7_pad(plaintext, BLOCK_SIZE);
        let mut output = Vec::with_capacity(BLOCK_SIZE + padded.len());

        // Prepend IV to output
        output.extend_from_slice(&self.iv.to_be_bytes());

        let mut prev_block = self.iv.to_block();

        for chunk in padded.chunks_exact(BLOCK_SIZE) {
            // chunks_exact guarantees exactly BLOCK_SIZE bytes
            let plain_block = Block128::from_be_bytes(chunk.try_into().expect("exact chunk size"));
            let xored = plain_block ^ prev_block.as_u128();
            let encrypted = self.aes.encrypt_block(xored);
            output.extend_from_slice(&encrypted.to_be_bytes());
            prev_block = encrypted;
        }

        Ok(output)
    }

    /// Decrypts ciphertext using CBC mode and removes PKCS#7 padding.
    ///
    /// Expects input format: `[16-byte IV][ciphertext...]`
    /// The IV is extracted from the input; the IV stored in `self` is ignored.
    ///
    /// # Errors
    ///
    /// Returns `CipherError::InvalidBlockSize` if input length is not a multiple of 16
    /// or is less than 32 bytes (IV + at least one block).
    /// Returns `CipherError::InvalidPadding` if padding is invalid.
    #[allow(clippy::missing_panics_doc)]
    pub fn decrypt(&self, data: &[u8]) -> CipherResult<Vec<u8>> {
        // Need at least IV (16 bytes) + one ciphertext block (16 bytes)
        if data.len() < BLOCK_SIZE * 2 || !data.len().is_multiple_of(BLOCK_SIZE) {
            return Err(CipherError::invalid_block_size(BLOCK_SIZE, data.len()));
        }

        // Extract IV from first block
        let iv = Iv::from_be_bytes(data[..BLOCK_SIZE].try_into().expect("exact IV size"));
        let ciphertext = &data[BLOCK_SIZE..];

        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut prev_block = iv.to_block();

        for chunk in ciphertext.chunks_exact(BLOCK_SIZE) {
            // chunks_exact guarantees exactly BLOCK_SIZE bytes
            let cipher_block = Block128::from_be_bytes(chunk.try_into().expect("exact chunk size"));
            let decrypted = self.aes.decrypt_block(cipher_block);
            let plain_block = decrypted ^ prev_block.as_u128();
            plaintext.extend_from_slice(&plain_block.to_be_bytes());
            prev_block = cipher_block;
        }

        let unpadded = pkcs7_unpad(&plaintext, BLOCK_SIZE)?;
        Ok(unpadded.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = 0x2b7e_1516_28ae_d2a6_abf7_1588_09cf_4f3c_u128;
        let iv = Iv::new(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f_u128);
        let cipher = AesCbc::new(key, iv);

        let plaintext = b"Hello, World!";
        let ciphertext = assert_ok!(cipher.encrypt(plaintext));
        let decrypted = assert_ok!(cipher.decrypt(&ciphertext));

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_decrypt_exact_block() {
        let key = 0x2b7e_1516_28ae_d2a6_abf7_1588_09cf_4f3c_u128;
        let iv = Iv::new(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f_u128);
        let cipher = AesCbc::new(key, iv);

        let plaintext = [0u8; 16];
        let ciphertext = assert_ok!(cipher.encrypt(&plaintext));
        // 16 IV + 16 data + 16 padding = 48 bytes
        assert_eq!(ciphertext.len(), 48);

        let decrypted = assert_ok!(cipher.decrypt(&ciphertext));
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_decrypt_multiple_blocks() {
        let key = 0x2b7e_1516_28ae_d2a6_abf7_1588_09cf_4f3c_u128;
        let iv = Iv::new(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f_u128);
        let cipher = AesCbc::new(key, iv);

        let plaintext = b"The quick brown fox jumps over the lazy dog";
        let ciphertext = assert_ok!(cipher.encrypt(plaintext));
        let decrypted = assert_ok!(cipher.decrypt(&ciphertext));

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_invalid_length_fails() {
        let key = 0x2b7e_1516_28ae_d2a6_abf7_1588_09cf_4f3c_u128;
        let iv = Iv::new(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f_u128);
        let cipher = AesCbc::new(key, iv);

        let invalid = [0u8; 15];
        assert_err!(cipher.decrypt(&invalid));
    }
}
