mod args;

use crate::args::Args;
use aes::{AesCbc, Block128, Iv};
use cipher_factory::{Algorithm, OperationMode};
use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use std::fs::{self, File};
use std::io::{Write, stdout};
use std::str::FromStr;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    // Check if we're doing file-based CBC operation
    if args.input_file.is_some() && args.algorithm == Algorithm::AesCbc {
        process_cbc_file(&args)?;
    } else {
        process_text(&args)?;
    }

    Ok(())
}

fn process_text(args: &Args) -> Result<()> {
    let input_text = match (&args.text, &args.input_file) {
        (Some(text), None) => text.clone(),
        (None, Some(path)) => fs::read_to_string(path)?,
        (Some(_), Some(_)) => return Err(eyre!("Cannot specify both TEXT and --input-file")),
        (None, None) => return Err(eyre!("Must specify TEXT or --input-file")),
    };

    let context = args.clone().into_context(input_text);
    let output = context.process()?;

    write_output(args, output.as_bytes())?;
    Ok(())
}

fn process_cbc_file(args: &Args) -> Result<()> {
    let input_path = args
        .input_file
        .as_ref()
        .ok_or_else(|| eyre!("No input file"))?;
    let iv_str = args
        .iv
        .as_ref()
        .ok_or_else(|| eyre!("CBC mode requires --iv"))?;

    let key = Block128::from_str(&args.key).map_err(|e| eyre!("Invalid key: {e}"))?;
    let iv = Iv::from_str(iv_str).map_err(|e| eyre!("Invalid IV: {e}"))?;

    let cipher = AesCbc::new(key, iv);

    match args.operation {
        OperationMode::Encrypt => {
            let plaintext = fs::read(input_path)?;
            let ciphertext = cipher
                .encrypt(&plaintext)
                .map_err(|e| eyre!("Encryption failed: {e}"))?;

            if args.output_file.is_some() {
                // Write raw binary to file
                write_output(args, &ciphertext)?;
            } else {
                // Write hex to stdout
                let hex = ciphertext.iter().fold(String::new(), |mut acc, b| {
                    use std::fmt::Write;
                    let _ = write!(acc, "{b:02X}");
                    acc
                });
                println!("{hex}");
            }
        }
        OperationMode::Decrypt => {
            let ciphertext = fs::read(input_path)?;
            let plaintext = cipher
                .decrypt(&ciphertext)
                .map_err(|e| eyre!("Decryption failed: {e}"))?;

            write_output(args, &plaintext)?;
        }
    }

    Ok(())
}

fn write_output(args: &Args, data: &[u8]) -> Result<()> {
    if let Some(path) = &args.output_file {
        let mut file = File::create(path)?;
        file.write_all(data)?;
    } else {
        stdout().write_all(data)?;
        // Add newline if output doesn't end with one
        if !data.ends_with(b"\n") {
            println!();
        }
    }
    Ok(())
}
