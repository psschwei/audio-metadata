# Audio Metadata Tool

A command-line tool for managing audio file metadata, supporting both MP3 and FLAC files. This tool uses `ffmpeg` to handle metadata operations, ensuring compatibility across different audio formats.

## Features

- Set cover art for audio files
- Process individual files or entire directories
- Automatic backup of original files
- Support for multiple audio formats (MP3, FLAC, M4A, OGG)
- Safe operation with automatic rollback on failure

## Prerequisites

- Rust (latest stable version)
- ffmpeg installed on your system

## Installation

1. Clone the repository:
```bash
git clone https://github.com/psschwei/audio-metadata.git
cd audio-metadata
```

2. Build the project:
```bash
cargo build --release
```

## Usage

The tool can process both individual files and entire directories of audio files.

### Setting Cover Art

#### For a Single File
```bash
cargo run -- set -f path/to/audio.mp3 -c path/to/cover.jpg
```

#### For a Directory of Files
```bash
cargo run -- set -f path/to/audio/directory -c path/to/cover.jpg
```

The tool will:
1. Create a backup of the original file(s)
2. Add the cover art to the audio file(s)
3. Show progress for each file being processed
4. Display the location of backup files

### Command Options

- `-f, --file`: Path to the audio file or directory
- `-c, --cover`: Path to the cover art image file

## Backup Files

When processing files, the tool creates backups in a temporary directory:
- For single files: `/tmp/audio-metadata-{timestamp}/`
- For directories: All files are backed up in a single `/tmp/audio-metadata-{timestamp}/` directory

You can safely delete these backup directories after verifying that the metadata changes are correct.

## Error Handling

- If an error occurs during processing, the original file is automatically restored
- For directory processing, if one file fails, the tool continues with the remaining files
- Error messages are displayed for any files that couldn't be processed

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Supported File Formats

The tool uses ffmpeg for metadata operations, so it supports all audio formats that ffmpeg can handle, including:
- MP3
- FLAC
- M4A
- OGG
- And many more

## Notes

- The tool creates a backup of your original file in `/tmp/audio-metadata-{timestamp}/`
- The backup location is printed to the screen
- You can safely delete the backup when you're satisfied with the changes
- The tool uses ffmpeg's ID3v2.3 format for metadata