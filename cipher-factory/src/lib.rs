//! Cipher factory for creating and configuring block ciphers.
//!
//! Provides a unified interface for AES and DES encryption/decryption
//! with configurable output formats.

mod algorithm;
mod context;
mod operation;
mod output;

pub use {
    algorithm::Algorithm, context::CipherContext, operation::OperationMode, output::OutputFormat,
};

pub mod prelude {
    pub use super::{Algorithm, CipherContext, OperationMode, OutputFormat};
}
