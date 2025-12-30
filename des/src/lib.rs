//! DES (Data Encryption Standard) implementation.
//!
//! Provides the classic DES block cipher with 64-bit keys and blocks.
//! Uses 16 Feistel rounds with 48-bit subkeys.
//!
//! # Example
//! ```
//! use des::Des;
//! use cipher_core::BlockCipher;
//!
//! let cipher = Des::new(0x133457799BBCDFF1_u64);
//! let ciphertext = cipher.encrypt(&[0u8; 8]).unwrap();
//! ```

mod block;
pub mod constants;
mod des;
mod key;
pub mod utils;

pub use {block::Block64, block::LR, des::Des};
