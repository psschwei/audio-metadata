use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set metadata for an audio file or directory
    Set {
        /// Path to the audio file or directory
        #[arg(short, long)]
        file: String,

        /// Path to cover art image
        #[arg(short, long)]
        cover: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { file, cover } => {
            let path = PathBuf::from(file);
            
            if let Some(cover_path) = cover {
                if path.is_dir() {
                    // Create a single temp directory for all files
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
                    fs::create_dir(&temp_dir)
                        .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;
                    
                    process_directory(&path, &PathBuf::from(cover_path), &temp_dir)?;
                    
                    println!("\nAll files have been processed.");
                    println!("Original files are backed up in: {}", temp_dir.display());
                    println!("You can safely delete the backup directory when you're satisfied with the changes.");
                } else {
                    set_cover_art(&path, &PathBuf::from(cover_path))?;
                }
            }
        }
    }

    Ok(())
}

fn process_directory(dir_path: &PathBuf, cover_path: &PathBuf, temp_dir: &PathBuf) -> Result<()> {
    let supported_extensions = ["mp3", "flac", "m4a", "ogg"];
    
    for entry in fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path.display()))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        println!("\nProcessing: {}", path.display());
                        if let Err(e) = set_cover_art_with_temp(&path, cover_path, temp_dir) {
                            eprintln!("Error processing {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn set_cover_art_with_temp(audio_path: &PathBuf, cover_path: &PathBuf, temp_dir: &PathBuf) -> Result<()> {
    // Copy original file to temp directory
    let backup_path = temp_dir.join(audio_path.file_name().unwrap());
    fs::copy(audio_path, &backup_path)
        .with_context(|| "Failed to copy original file to temp directory")?;
    
    println!("Original file backed up to: {}", backup_path.display());

    // Use ffmpeg to copy the audio and add the cover art
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel", "error",
            "-i", backup_path.to_str().unwrap(),
            "-i", cover_path.to_str().unwrap(),
            "-map", "0:a",
            "-map", "1:v",
            "-c:a", "copy",
            "-c:v", "copy",
            "-id3v2_version", "3",
            "-metadata:s:v", "title=Cover (front)",
            "-metadata:s:v", "comment=Cover (front)",
            audio_path.to_str().unwrap(),
        ])
        .status()
        .with_context(|| "Failed to execute ffmpeg command")?;

    if !status.success() {
        // If ffmpeg failed, restore the original file
        fs::copy(&backup_path, audio_path)
            .with_context(|| "Failed to restore original file after ffmpeg error")?;
        return Err(anyhow::anyhow!("ffmpeg command failed"));
    }

    println!("Successfully updated cover art for {}", audio_path.display());

    Ok(())
}

fn set_cover_art(audio_path: &PathBuf, cover_path: &PathBuf) -> Result<()> {
    // Create a temp directory with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
    fs::create_dir(&temp_dir)
        .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;

    set_cover_art_with_temp(audio_path, cover_path, &temp_dir)?;

    println!("Original file is backed up at: {}", temp_dir.join(audio_path.file_name().unwrap()).display());
    println!("You can safely delete the backup when you're satisfied with the changes.");

    Ok(())
}
