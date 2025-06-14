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

        /// Song title to set
        #[arg(short = 't', long)]
        title: Option<String>,

        /// Track number to set
        #[arg(short = 'n', long)]
        track: Option<u32>,

        /// Infer track name from filename (removes track numbers and file extension)
        #[arg(long)]
        infer_track: bool,

        /// Infer track numbers based on sorted order of files in directory
        #[arg(long)]
        infer_order: bool,
    },

    /// Convert FLAC files to MP3
    Convert {
        /// Path to the FLAC file or directory
        #[arg(short, long)]
        file: String,

        /// Output directory (defaults to same directory as input)
        #[arg(short, long)]
        output: Option<String>,

        /// MP3 bitrate in kbps (default: 320)
        #[arg(short, long, default_value = "320")]
        bitrate: u32,
    },
} 