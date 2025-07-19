// src/main.rs
/*
 * Main executable for FintechsolutionsSystems
 */

use clap::Parser;
use fintechsolutionssystems::{Result, run};

#[derive(Parser)]
#[command(version, about = FintechsolutionsSystems - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
