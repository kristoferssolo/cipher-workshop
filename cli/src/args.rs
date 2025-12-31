use cipher_factory::{Algorithm, CipherContext, OperationMode, OutputFormat};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Operation to perform
    #[arg(value_name = "OPERATION")]
    pub operation: OperationMode,

    /// Encryption algorithm
    #[arg(short, long)]
    pub algorithm: Algorithm,

    /// Key used for encryption/decryption (hex string, e.g., 0x2b7e...)
    #[arg(short, long, required = true)]
    pub key: String,

    /// Initialization vector for CBC mode (hex string, e.g., 0x0001...)
    #[arg(long)]
    pub iv: Option<String>,

    /// The text to encrypt/decrypt (use --input-file for file input)
    #[arg(value_name = "TEXT", required_unless_present = "input_file")]
    pub text: Option<String>,

    /// Input file to encrypt/decrypt
    #[arg(short, long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    /// Output file (defaults to stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output_file: Option<PathBuf>,

    /// Output format for decrypted data
    #[arg(short = 'f', long)]
    pub output_format: Option<OutputFormat>,
}

impl Args {
    /// Creates a [`CipherContext`] for text-based operations.
    #[must_use]
    pub fn into_context(self, input_text: String) -> CipherContext {
        CipherContext {
            algorithm: self.algorithm,
            operation: self.operation,
            key: self.key,
            iv: self.iv,
            input_text,
            output_format: self.output_format.unwrap_or_default(),
        }
    }
}
