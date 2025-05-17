use anyhow::{Result, Context};
use std::path::PathBuf;
use std::fs;
use crate::metadata;

/// Process a directory of audio files, setting cover art, album title, artist, and/or song title
pub fn process_directory(
    dir_path: &PathBuf,
    cover_path: Option<PathBuf>,
    album_title: Option<&str>,
    artist: Option<&str>,
    title: Option<&str>,
    temp_dir: &PathBuf
) -> Result<()> {
    let supported_extensions = ["mp3", "flac"];
    let mut error_count = 0;
    
    for entry in fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path.display()))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        // Backup the file
                        let backup_path = temp_dir.join(path.file_name().unwrap());
                        if let Err(e) = fs::copy(&path, &backup_path) {
                            eprintln!("Error backing up {}: {}", path.display(), e);
                            error_count += 1;
                            continue;
                        }

                        // Set cover art if provided
                        if let Some(ref cover) = cover_path {
                            if let Err(e) = metadata::set_cover_art_with_temp(&path, cover, temp_dir) {
                                eprintln!("Error setting cover art for {}: {}", path.display(), e);
                                error_count += 1;
                            }
                        }

                        // Set album title if provided
                        if let Some(album) = album_title {
                            if let Err(e) = metadata::set_album_title(&path, album) {
                                eprintln!("Error setting album title for {}: {}", path.display(), e);
                                error_count += 1;
                            }
                        }

                        // Set artist if provided
                        if let Some(artist_name) = artist {
                            if let Err(e) = metadata::set_artist(&path, artist_name) {
                                eprintln!("Error setting artist for {}: {}", path.display(), e);
                                error_count += 1;
                            }
                        }

                        // Set song title if provided
                        if let Some(song_title) = title {
                            if let Err(e) = metadata::set_title_with_temp(&path, song_title, temp_dir) {
                                eprintln!("Error setting song title for {}: {}", path.display(), e);
                                error_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    if error_count > 0 {
        println!("\nCompleted with {} errors. Check the messages above for details.", error_count);
    }

    Ok(())
}

/// Process a directory of FLAC files, converting them to MP3
pub fn process_directory_conversion(
    dir_path: &PathBuf,
    output_dir: Option<&PathBuf>,
    bitrate: u32,
    temp_dir: &PathBuf
) -> Result<()> {
    let mut error_count = 0;
    
    for entry in fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path.display()))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if ext_str.to_lowercase() == "flac" {
                        // Determine output path
                        let output_path = if let Some(dir) = output_dir {
                            dir.join(path.file_stem().unwrap()).with_extension("mp3")
                        } else {
                            path.with_extension("mp3")
                        };

                        // Convert the file
                        if let Err(e) = metadata::convert_to_mp3(&path, &output_path, bitrate, temp_dir) {
                            eprintln!("Error converting {}: {}", path.display(), e);
                            error_count += 1;
                        }
                    }
                }
            }
        }
    }

    if error_count > 0 {
        println!("\nCompleted with {} errors. Check the messages above for details.", error_count);
    }

    Ok(())
} 