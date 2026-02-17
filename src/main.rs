mod cli;
mod decode;
mod encode;

use clap::Parser;
use cli::Cli;

use crate::cli::Commands;
use anyhow::Result;
use decode::handle as handle_decode;
use encode::handle as handle_encode;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => handle_encode(args),
        Commands::Decode(args) => handle_decode(args),
    }
}
