use crate::{output::OutputFormat, value::Value};
use clap::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Operation to perform
    #[arg(value_name = "OPERATION")]
    pub operation: OperationChoice,

    /// Encryption algorithm
    #[arg(short, long)]
    pub algorithm: AlgorithmChoice,

    /// Key used for encryption/decryption. Can be a string or a path to a file
    #[arg(short, long, value_parser = Value::from_str, required = true)]
    pub key: Value,

    /// The text to encrypt/decrypt. Can be a string or a path to a file
    #[arg(value_name = "TEXT", value_parser = Value::from_str, required = true)]
    pub text: Value,

    /// Output format for decrypted data
    #[arg(short = 'f', long)]
    pub output_format: Option<OutputFormat>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AlgorithmChoice {
    Des,
    Aes,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OperationChoice {
    Encrypt,
    Decrypt,
}
