
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};

/// Program to encode file as one of more directories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Encode or decode file
	#[command(subcommand)]
    pub mode: Mode,

	/// Force encoding/decoding to overrite the files or directory completely
	/// Use at your own risk!!!
	#[arg(short, long, action)]
	pub force: bool
}

#[derive(Subcommand, Debug)]
pub enum Mode {
	Encode {
		#[arg(value_hint = ValueHint::FilePath)]
		file: PathBuf,
		#[arg(value_hint = ValueHint::DirPath)]
		directory: PathBuf,
	},
	Decode {
		#[arg(value_hint = ValueHint::DirPath)]
		directory: PathBuf,
		#[arg(value_hint = ValueHint::FilePath)]
		file: PathBuf,
	},
}
