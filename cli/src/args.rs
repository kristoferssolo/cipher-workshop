use cipher_factory::{Algorithm, OperationChoice, OutputFormat};
use clap::Parser;
use des::Block64;
use std::str::FromStr;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Operation to perform
    #[arg(value_name = "OPERATION")]
    pub operation: OperationChoice,

    /// Encryption algorithm
    #[arg(short, long)]
    pub algorithm: Algorithm,

    /// Key used for encryption/decryption. Can be a string or a path to a file
    #[arg(short, long, value_parser = Block64::from_str, required = true)]
    pub key: Block64,

    /// The text to encrypt/decrypt. Can be a string or a path to a file
    #[arg(value_name = "TEXT", value_parser = Block64::from_str, required = true)]
    pub text: Block64,

    /// Output format for decrypted data
    #[arg(short = 'f', long)]
    pub output_format: Option<OutputFormat>,
}
