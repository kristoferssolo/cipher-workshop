mod args;

use crate::args::{Args, Operation, OutputFormat};
use cipher_core::BlockCipher;
use clap::Parser;
use des::Des;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let des = Des::new(args.key.as_64());

    match args.operation {
        Operation::Encrypt => {
            let ciphertext = des.encrypt(&args.text.to_be_bytes())?;
            println!("{ciphertext:016X}");
        }
        Operation::Decrypt { output_format } => {
            let plaintext = des.decrypt(&args.text.to_be_bytes())?;
            match output_format.unwrap_or_default() {
                OutputFormat::Binary => println!("{plaintext:064b}"),
                OutputFormat::Octal => println!("{plaintext:022o}"),
                OutputFormat::Decimal => println!("{plaintext}"),
                OutputFormat::Hex => println!("{plaintext:016X}"),
                OutputFormat::Text => println!("{plaintext}"),
            }
        }
    }
    Ok(())
}
