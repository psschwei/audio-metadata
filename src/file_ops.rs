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
    track: Option<u32>,
    infer_track: bool,
    infer_order: bool,
    temp_dir: &PathBuf
) -> Result<()> {
    let supported_extensions = ["mp3", "flac"];
    let mut error_count = 0;
    
    // If infer_order is enabled, collect and sort files first
    if infer_order {
        let mut audio_files: Vec<PathBuf> = Vec::new();
        
        for entry in fs::read_dir(dir_path)
            .with_context(|| format!("Failed to read directory: {}", dir_path.display()))? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            audio_files.push(path);
                        }
                    }
                }
            }
        }
        
        // Sort files by name
        audio_files.sort();
        
        // Process files with track numbers
        for (index, path) in audio_files.iter().enumerate() {
            let track_number = index + 1;
            
            // Backup the file
            let backup_path = temp_dir.join(path.file_name().unwrap());
            if let Err(e) = fs::copy(path, &backup_path) {
                eprintln!("Error backing up {}: {}", path.display(), e);
                error_count += 1;
                continue;
            }

            // Set track number
            if let Err(e) = metadata::set_track_number(path, track_number as u32) {
                eprintln!("Error setting track number for {}: {}", path.display(), e);
                error_count += 1;
            } else {
                println!("Set track number {} for {}", track_number, path.display());
            }

            // Process other metadata operations
            process_single_file_metadata(path, &cover_path, album_title, artist, title, track, infer_track, temp_dir, &mut error_count)?;
        }
    } else {
        // Process files without track number inference (original behavior)
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

                            process_single_file_metadata(&path, &cover_path, album_title, artist, title, track, infer_track, temp_dir, &mut error_count)?;
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

/// Process metadata for a single audio file
fn process_single_file_metadata(
    path: &PathBuf,
    cover_path: &Option<PathBuf>,
    album_title: Option<&str>,
    artist: Option<&str>,
    title: Option<&str>,
    track: Option<u32>,
    infer_track: bool,
    temp_dir: &PathBuf,
    error_count: &mut i32
) -> Result<()> {
    // Set cover art if provided
    if let Some(cover) = cover_path {
        if let Err(e) = metadata::set_cover_art_with_temp(path, cover, temp_dir) {
            eprintln!("Error setting cover art for {}: {}", path.display(), e);
            *error_count += 1;
        }
    }

    // Set album title if provided
    if let Some(album) = album_title {
        if let Err(e) = metadata::set_album_title(path, album) {
            eprintln!("Error setting album title for {}: {}", path.display(), e);
            *error_count += 1;
        }
    }

    // Set artist if provided
    if let Some(artist_name) = artist {
        if let Err(e) = metadata::set_artist(path, artist_name) {
            eprintln!("Error setting artist for {}: {}", path.display(), e);
            *error_count += 1;
        }
    }

    // Set song title if provided
    if let Some(song_title) = title {
        if let Err(e) = metadata::set_title_with_temp(path, song_title, temp_dir) {
            eprintln!("Error setting song title for {}: {}", path.display(), e);
            *error_count += 1;
        }
    }

    // Set track number if provided
    if let Some(track_number) = track {
        if let Err(e) = metadata::set_track_number(path, track_number) {
            eprintln!("Error setting track number for {}: {}", path.display(), e);
            *error_count += 1;
        } else {
            println!("Set track number {} for {}", track_number, path.display());
        }
    }

    // Infer and set track name from filename if requested
    if infer_track {
        match metadata::infer_track_name_from_filename(path) {
            Ok(inferred_title) => {
                if let Err(e) = metadata::set_title_with_temp(path, &inferred_title, temp_dir) {
                    eprintln!("Error setting inferred track name for {}: {}", path.display(), e);
                    *error_count += 1;
                } else {
                    println!("Set track name to '{}' for {}", inferred_title, path.display());
                }
            }
            Err(e) => {
                eprintln!("Error inferring track name for {}: {}", path.display(), e);
                *error_count += 1;
            }
        }
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