mod cli;
mod file_ops;
mod metadata;

use anyhow::{Result, Context};
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::{Cli, Commands};
use crate::file_ops::process_directory;
use crate::metadata::{set_album_title, set_artist, set_cover_art};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { file, cover, album, artist } => {
            let path = PathBuf::from(file);
            
            if path.is_dir() {
                // Create a single temp directory for all files
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
                fs::create_dir(&temp_dir)
                    .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;
                
                process_directory(
                    &path,
                    cover.as_ref().map(PathBuf::from),
                    album.as_deref(),
                    artist.as_deref(),
                    &temp_dir
                )?;
                
                println!("\nAll files have been processed.");
                println!("Original files are backed up in: {}", temp_dir.display());
                println!("You can safely delete the backup directory when you're satisfied with the changes.");
            } else {
                if let Some(cover_path) = cover {
                    set_cover_art(&path, &PathBuf::from(cover_path))?;
                }
                if let Some(album_title) = album {
                    set_album_title(&path, &album_title)?;
                }
                if let Some(artist_name) = artist {
                    set_artist(&path, &artist_name)?;
                }
            }
        }
    }

    Ok(())
}
