mod args;

use crate::args::Args;
use cipher_core::BlockCipher;
use cipher_factory::{self, OperationChoice, OutputFormat};
use clap::Parser;
use color_eyre::eyre::{Ok, Result};

fn main() -> Result<()> {
    color_eyre::install()?;
    let Args {
        operation,
        algorithm,
        key,
        text,
        output_format,
    } = Args::parse();

    let text_bytes = algorithm.parse_text(&text)?;
    let cipher = algorithm.new_cipher(&key)?;

    execute_cipher(operation, cipher.as_ref(), &text_bytes, output_format)?;

    Ok(())
}

fn execute_cipher(
    operation: OperationChoice,
    cipher: &dyn BlockCipher,
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
