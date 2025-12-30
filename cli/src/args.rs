use cipher_factory::{Algorithm, CipherContext, OperationMode, OutputFormat};
use clap::Parser;

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

    /// The text to encrypt/decrypt
    #[arg(value_name = "TEXT", required = true)]
    pub text: String,

    /// Output format for decrypted data
    #[arg(short = 'f', long)]
    pub output_format: Option<OutputFormat>,
}

impl From<Args> for CipherContext {
    fn from(args: Args) -> Self {
        Self {
            algorithm: args.algorithm,
            operation: args.operation,
            key: args.key,
            iv: args.iv,
            input_text: args.text,
            output_format: args.output_format.unwrap_or_default(),
        }
    }
}
