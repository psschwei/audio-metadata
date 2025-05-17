use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set metadata for an audio file or directory
    Set {
        /// Path to the audio file or directory
        #[arg(short, long)]
        file: String,

        /// Path to cover art image
        #[arg(short, long)]
        cover: Option<String>,

        /// Album title to set
        #[arg(short, long)]
        album: Option<String>,

        /// Artist name to set
        #[arg(short = 'r', long)]
        artist: Option<String>,
    },
} 