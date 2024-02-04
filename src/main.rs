use std::error::Error;

use clap::Parser;
use directory_encoding::{cli::{Cli, Mode}, encode, decode};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if let Err(e) = match cli.mode {
        Mode::Encode { .. } => encode(cli),
        Mode::Decode { .. } => decode(cli),
    } {
        eprintln!("Error: {e}");
    }
    
    Ok(())
}
