use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file path
    pub input_file: PathBuf,
    /// Address for first instruction
    #[arg(short, long, default_value_t = 0)]
    pub base_address: usize,
    /// Output file
    #[arg(short, long, value_name = "FILE", default_value = "output.txt")]
    pub output: PathBuf,
}
