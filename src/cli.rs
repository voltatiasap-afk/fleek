use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// encodes files or text to an image
    Encode(EncodeArgs),

    /// decodes hidden data from images
    Decode(DecodeArgs),
}

#[derive(Args)]
pub struct DecodeArgs {
    #[arg(short, long)]
    pub image: String,
}

#[derive(Args)]
pub struct EncodeArgs {
    #[arg(short, long)]
    pub mask: String,

    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    // #[command(subcommand)]
    #[arg(short, long, conflicts_with = "file")]
    pub text: Option<String>,

    #[arg(short, long, conflicts_with = "text")]
    pub file: Option<String>,

    #[arg(short, long, default_value = "0", value_parser = clap::value_parser!(u32).range(0..=10))]
    pub compression: u32,
}

#[derive(Subcommand)]
pub enum Source {
    /// Encode text data into carrier
    #[command(name = "text")]
    Text { text: String },

    /// Encode a file into carrier
    #[command(name = "file")]
    File { file: String },
}
