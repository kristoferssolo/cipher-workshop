mod args;

use crate::args::Args;
use aes::{Aes, Block128};
use cipher_core::BlockCipher;
use cipher_factory::{Algorithm, OperationChoice, OutputFormat};
use clap::Parser;
use color_eyre::eyre::{Ok, Result};
use des::{Block64, Des};
use std::str::FromStr;

fn main() -> Result<()> {
    color_eyre::install()?;
    let Args {
        operation,
        algorithm,
        key,
        text,
        output_format,
    } = Args::parse();

    match algorithm {
        Algorithm::Des => {
            let key = Block64::from_str(&key)?;
            let text = Block64::from_str(&text)?;
            let cipher = Des::from_key(&key.to_be_bytes());
            execute_cipher(operation, &cipher, &text.to_be_bytes(), output_format)?;
        }
        Algorithm::Aes => {
            let key = Block128::from_str(&key)?;
            let text = Block128::from_str(&text)?;
            let cipher = Aes::from_key(&key.to_be_bytes());
            execute_cipher(operation, &cipher, &text.to_be_bytes(), output_format)?;
        }
    }
    Ok(())
}

fn execute_cipher(
    operation: OperationChoice,
    cipher: &impl BlockCipher,
    text_bytes: &[u8],
    output_format: Option<OutputFormat>,
) -> Result<()> {
    match operation {
        OperationChoice::Encrypt => {
            let ciphertext = cipher.encrypt(text_bytes)?;
            println!("{ciphertext:X}");
        }
        OperationChoice::Decrypt => {
            let plaintext = cipher.decrypt(text_bytes)?;
            let output = output_format.unwrap_or_default().to_string(&plaintext);
            println!("{output}");
        }
    }

    Ok(())
}
