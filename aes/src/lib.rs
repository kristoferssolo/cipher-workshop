//! AES (Advanced Encryption Standard) implementation.
//!
//! Provides AES-128 block cipher with 128-bit keys and blocks.
//!
//! # Example
//! ```
//! use aes::Aes;
//! use cipher_core::BlockCipher;
//!
//! let cipher = Aes::new(0x2b7e1516_28aed2a6_abf71588_09cf4f3c_u128);
//! let ciphertext = cipher.encrypt(&[0u8; 16]).unwrap();
//! ```

mod aes;
mod block;
mod constants;
mod iv;
mod key;
mod operations;
mod sbox;

pub use {aes::Aes, block::Block32, block::Block128, iv::Iv};
