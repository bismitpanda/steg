use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Subcommand)]
pub enum Actions {
    /// Encode a message in to an image
    #[command(visible_alias = "e")]
    Encode(EncodeArgs),

    /// Extract the message stored in a message
    #[command(visible_alias = "d")]
    Decode(DecodeArgs),

    /// Calculate the possible storage capacity of the image
    #[command(visible_alias = "c")]
    Calculate(CalculateArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    /// The key to encrypt the data
    #[arg(short, long)]
    pub key: Option<String>,

    /// The image to be used for storage
    #[arg(short = 'p', long)]
    pub image_path: PathBuf,

    /// The input file to read
    #[arg(short, long)]
    pub infile: PathBuf,

    /// The output file to write
    #[arg(short, long)]
    pub outfile: PathBuf,
}

#[derive(Args)]
pub struct DecodeArgs {
    /// The key to decrypt the data
    #[arg(short, long)]
    pub key: Option<String>,

    /// The input file to read
    #[arg(short, long)]
    pub infile: PathBuf,

    /// The output file to write
    #[arg(short, long)]
    pub outfile: PathBuf,
}

#[derive(Args)]
pub struct CalculateArgs {
    /// The image to be used for storage
    #[arg(short = 'p', long)]
    pub image_path: PathBuf,
}

#[derive(Parser)]
#[command(
    about = "This is a utility to hide files inside image files in an encrypted format.
Currently supports: PNG, TIFF, BMP, ICO"
)]
pub struct Command {
    #[command(subcommand)]
    pub action: Actions,
}
