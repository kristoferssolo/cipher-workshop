mod args;

use crate::args::Args;
use cipher_core::BlockCipher;
use cipher_factory::OperationChoice;
use clap::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let Args {
        operation,
        algorithm,
        key,
        text,
        output_format,
    } = Args::parse();

    match operation {
        OperationChoice::Encrypt => {
            let cipher = algorithm.get_cipher(&key);
            let ciphertext = cipher.encrypt(&text.to_be_bytes())?;
            println!("{ciphertext:016X}");
        }
        OperationChoice::Decrypt => {
            let cipher = algorithm.get_cipher(&key);
            let plaintext = cipher.decrypt(&text.to_be_bytes())?;
            let output = output_format.unwrap_or_default().to_string(&plaintext);
            println!("{output}");
        }
    }
    Ok(())
}
