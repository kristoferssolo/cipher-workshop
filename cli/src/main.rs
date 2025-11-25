mod args;

use crate::args::Args;
use cipher_factory::CipherContext;
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let context = CipherContext::from(args);

    let output = context.process()?;
    println!("{output}");

    Ok(())
}
