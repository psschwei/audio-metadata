mod cli;
mod file_ops;
mod metadata;

use anyhow::{Result, Context};
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::{Cli, Commands};
use crate::file_ops::{process_directory, process_directory_conversion};
use crate::metadata::{convert_flac_to_mp3};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { file, cover, album, artist, title, infer_track } => {
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
                    title.as_deref(),
                    infer_track,
                    &temp_dir
                )?;
                
                println!("\nAll files have been processed.");
                println!("Original files are backed up in: {}", temp_dir.display());
                println!("You can safely delete the backup directory when you're satisfied with the changes.");
            } else {
                // Create a temp directory for the single file
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
                fs::create_dir(&temp_dir)
                    .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;

                // Process each metadata operation
                if let Some(cover_path) = cover {
                    metadata::set_cover_art_with_temp(&path, &PathBuf::from(cover_path), &temp_dir)?;
                }
                if let Some(album_title) = album {
                    metadata::set_album_title(&path, &album_title)?;
                }
                if let Some(artist_name) = artist {
                    metadata::set_artist(&path, &artist_name)?;
                }
                if let Some(song_title) = title {
                    metadata::set_title_with_temp(&path, &song_title, &temp_dir)?;
                }
                if infer_track {
                    let inferred_title = metadata::infer_track_name_from_filename(&path)?;
                    metadata::set_title_with_temp(&path, &inferred_title, &temp_dir)?;
                }

                println!("\nFile has been processed.");
                println!("Original file is backed up in: {}", temp_dir.display());
                println!("You can safely delete the backup directory when you're satisfied with the changes.");
            }
        }
        Commands::Convert { file, output, bitrate } => {
            let input_path = PathBuf::from(file);
            let output_dir = output.map(PathBuf::from);

            // Create a temp directory for backups
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
            fs::create_dir(&temp_dir)
                .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;

            if input_path.is_dir() {
                // Create output directory if specified
                if let Some(ref dir) = output_dir {
                    fs::create_dir_all(dir)
                        .with_context(|| format!("Failed to create output directory: {}", dir.display()))?;
                }

                process_directory_conversion(&input_path, output_dir.as_ref(), bitrate, &temp_dir)?;
                
                println!("\nAll files have been processed.");
                println!("Original files are backed up in: {}", temp_dir.display());
                println!("You can safely delete the backup directory when you're satisfied with the conversions.");
            } else {
                // Verify it's a FLAC file
                if let Some(ext) = input_path.extension() {
                    if ext.to_str().unwrap().to_lowercase() != "flac" {
                        return Err(anyhow::anyhow!("Input file must be a FLAC file"));
                    }
                } else {
                    return Err(anyhow::anyhow!("Input file must have an extension"));
                }

                // Create output directory if specified
                if let Some(ref dir) = output_dir {
                    fs::create_dir_all(dir)
                        .with_context(|| format!("Failed to create output directory: {}", dir.display()))?;
                }

                convert_flac_to_mp3(&input_path, output_dir.as_ref(), bitrate)?;
            }
        }
    }

    Ok(())
}
