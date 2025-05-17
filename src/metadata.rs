use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Set the artist metadata for an audio file
pub fn set_artist(file_path: &PathBuf, artist: &str) -> Result<()> {
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension"))?;

    match extension.to_lowercase().as_str() {
        "flac" => set_flac_artist(file_path, artist),
        "mp3" => set_mp3_artist(file_path, artist),
        _ => Err(anyhow::anyhow!("Unsupported file format: {}", extension)),
    }
}

fn set_flac_artist(file_path: &PathBuf, artist: &str) -> Result<()> {
    // First remove any existing ARTIST tag
    let status = Command::new("metaflac")
        .args(["--remove-tag", "ARTIST", file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to remove existing tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while removing existing tag"));
    }

    // Then set the new ARTIST tag
    let status = Command::new("metaflac")
        .args(["--set-tag", &format!("ARTIST={}", artist), file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to set new tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while setting new tag"));
    }

    Ok(())
}

fn set_mp3_artist(file_path: &PathBuf, artist: &str) -> Result<()> {
    let status = Command::new("id3v2")
        .args(["--artist", artist, file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute id3v2 command")?;

    if !status.success() {
        return Err(anyhow::anyhow!("id3v2 command failed"));
    }

    Ok(())
}

/// Set the album title metadata for an audio file
pub fn set_album_title(file_path: &PathBuf, album_title: &str) -> Result<()> {
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension"))?;

    match extension.to_lowercase().as_str() {
        "flac" => set_flac_album_title(file_path, album_title),
        "mp3" => set_mp3_album_title(file_path, album_title),
        _ => Err(anyhow::anyhow!("Unsupported file format: {}", extension)),
    }
}

fn set_flac_album_title(file_path: &PathBuf, album_title: &str) -> Result<()> {
    // First remove any existing ALBUM tag
    let status = Command::new("metaflac")
        .args(["--remove-tag", "ALBUM", file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to remove existing tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while removing existing tag"));
    }

    // Then set the new ALBUM tag
    let status = Command::new("metaflac")
        .args(["--set-tag", &format!("ALBUM={}", album_title), file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to set new tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while setting new tag"));
    }

    Ok(())
}

fn set_mp3_album_title(file_path: &PathBuf, album_title: &str) -> Result<()> {
    let status = Command::new("id3v2")
        .args(["--album", album_title, file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute id3v2 command")?;

    if !status.success() {
        return Err(anyhow::anyhow!("id3v2 command failed"));
    }

    Ok(())
}

/// Set the cover art for an audio file
pub fn set_cover_art(audio_path: &PathBuf, cover_path: &PathBuf) -> Result<()> {
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

/// Set the cover art for an audio file using a temporary directory
pub fn set_cover_art_with_temp(audio_path: &PathBuf, cover_path: &PathBuf, temp_dir: &PathBuf) -> Result<()> {
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

// ... existing code for cover art and album title ... 