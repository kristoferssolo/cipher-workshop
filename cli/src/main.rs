mod args;
mod cipher;
mod error;
mod output;
mod value;

use crate::{
    args::{Args, OperationChoice},
    output::OutputFormat,
};
use cipher_core::BlockCipher;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let Args {
        operation,
        algorithm,
        key,
        text,
        output_format,
    } = Args::parse();

    match operation {
        OperationChoice::Encrypt => {
            let cipher = algorithm.get_cipher(key);
            let ciphertext = cipher.encrypt(&text.to_be_bytes())?;
            println!("{ciphertext:016X}");
        }
        OperationChoice::Decrypt => {
            let cipher = algorithm.get_cipher(key);
            let plaintext = cipher.decrypt(&text.to_be_bytes())?;
            match output_format.unwrap_or_default() {
                OutputFormat::Binary => println!("{plaintext:064b}"),
                OutputFormat::Octal => println!("{plaintext:022o}"),
                OutputFormat::Hex => println!("{plaintext:016X}"),
                OutputFormat::Text => println!("{plaintext}"),
            }
        }
    }
    Ok(())
}
