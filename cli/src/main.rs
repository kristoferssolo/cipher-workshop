mod args;

use crate::args::{Args, Operation, OutputFormat};
use cipher_core::BlockCipher;
use clap::Parser;
use des::Des;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.operation {
        Operation::Encrypt { key, text } => {
            let des = Des::new(key.as_64());
            let ciphertext = des.encrypt(&text.to_be_bytes())?;
            println!("{ciphertext:016X}");
        }
        Operation::Decrypt {
            key,
            text,
            output_format,
        } => {
            let des = Des::new(key.as_64());
            let plaintext = des.decrypt(&text.to_be_bytes())?;
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
