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

/// Set the song title metadata for an audio file using a temporary directory
pub fn set_title_with_temp(file_path: &PathBuf, title: &str, temp_dir: &PathBuf) -> Result<()> {
    // Copy original file to temp directory
    let backup_path = temp_dir.join(file_path.file_name().unwrap());
    fs::copy(file_path, &backup_path)
        .with_context(|| "Failed to copy original file to temp directory")?;
    
    println!("Original file backed up to: {}", backup_path.display());

    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension"))?;

    let result = match extension.to_lowercase().as_str() {
        "flac" => set_flac_title(file_path, title),
        "mp3" => set_mp3_title(file_path, title),
        _ => Err(anyhow::anyhow!("Unsupported file format: {}", extension)),
    };

    if let Err(e) = result {
        // If setting title failed, restore the original file
        fs::copy(&backup_path, file_path)
            .with_context(|| "Failed to restore original file after error")?;
        return Err(e);
    }

    println!("Successfully updated song title for {}", file_path.display());

    Ok(())
}

fn set_flac_title(file_path: &PathBuf, title: &str) -> Result<()> {
    // First remove any existing TITLE tag
    let status = Command::new("metaflac")
        .args(["--remove-tag", "TITLE", file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to remove existing tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while removing existing tag"));
    }

    // Then set the new TITLE tag
    let status = Command::new("metaflac")
        .args(["--set-tag", &format!("TITLE={}", title), file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute metaflac command to set new tag")?;

    if !status.success() {
        return Err(anyhow::anyhow!("metaflac command failed while setting new tag"));
    }

    Ok(())
}

fn set_mp3_title(file_path: &PathBuf, title: &str) -> Result<()> {
    let status = Command::new("id3v2")
        .args(["--song", title, file_path.to_str().unwrap()])
        .status()
        .with_context(|| "Failed to execute id3v2 command")?;

    if !status.success() {
        return Err(anyhow::anyhow!("id3v2 command failed"));
    }

    Ok(())
}

/// Convert a FLAC file to MP3
pub fn convert_to_mp3(
    input_path: &PathBuf,
    output_path: &PathBuf,
    bitrate: u32,
    temp_dir: &PathBuf
) -> Result<()> {
    // Backup the original file
    let backup_path = temp_dir.join(input_path.file_name().unwrap());
    fs::copy(input_path, &backup_path)
        .with_context(|| "Failed to copy original file to temp directory")?;

    // Use ffmpeg to convert FLAC to MP3
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel", "error",
            "-i", input_path.to_str().unwrap(),
            "-codec:a", "libmp3lame",
            "-b:a", &format!("{}k", bitrate),
            "-map_metadata", "0",
            output_path.to_str().unwrap(),
        ])
        .status()
        .with_context(|| "Failed to execute ffmpeg command")?;

    if !status.success() {
        // If conversion failed, restore the original file
        fs::copy(&backup_path, input_path)
            .with_context(|| "Failed to restore original file after ffmpeg error")?;
        return Err(anyhow::anyhow!("ffmpeg command failed"));
    }

    Ok(())
}

/// Convert a FLAC file to MP3, preserving metadata
pub fn convert_flac_to_mp3(
    input_path: &PathBuf,
    output_dir: Option<&PathBuf>,
    bitrate: u32
) -> Result<()> {
    // Create a temp directory with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temp_dir = PathBuf::from(format!("/tmp/audio-metadata-{}", timestamp));
    fs::create_dir(&temp_dir)
        .with_context(|| format!("Failed to create temp directory: {}", temp_dir.display()))?;

    // Determine output path
    let output_path = if let Some(dir) = output_dir {
        dir.join(input_path.file_stem().unwrap()).with_extension("mp3")
    } else {
        input_path.with_extension("mp3")
    };

    // Convert the file
    convert_to_mp3(input_path, &output_path, bitrate, &temp_dir)?;

    println!("Successfully converted {} to {}", input_path.display(), output_path.display());
    println!("Original file is backed up at: {}", temp_dir.join(input_path.file_name().unwrap()).display());
    println!("You can safely delete the backup when you're satisfied with the conversion.");

    Ok(())
} 