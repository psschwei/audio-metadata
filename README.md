# Audio Metadata Tool

A command-line tool for managing audio file metadata, built in Rust. 

## Features

- Set album titles for audio files
- Set artist names for audio files
- Set song titles for audio files
- Add cover art to audio files
- Process entire directories of audio files
- Supports FLAC and MP3 formats
- Creates backups before modifying files

## Requirements

- Rust (for building)
- ffmpeg (for cover art operations)
- metaflac (for FLAC metadata)
- id3v2 (for MP3 metadata)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/audio-metadata.git
cd audio-metadata
```

2. Build the project:
```bash
cargo build --release
```

3. (Optional) Install the binary to your system:
```bash
sudo cp target/release/audio-metadata /usr/local/bin/
```

## Usage

The tool provides a `set` command for modifying metadata. You can either modify a single file or process an entire directory.

### Setting Album Title

For a single file:
```bash
audio-metadata set -f song.mp3 -a "Album Name"
```

For a directory:
```bash
audio-metadata set -f music_directory -a "Album Name"
```

### Setting Artist Name

For a single file:
```bash
audio-metadata set -f song.mp3 -r "Artist Name"
```

For a directory:
```bash
audio-metadata set -f music_directory -r "Artist Name"
```

### Setting Song Title

For a single file:
```bash
audio-metadata set -f song.mp3 -t "Song Title"
```

### Setting Cover Art

For a single file:
```bash
audio-metadata set -f song.mp3 -c cover.jpg
```

For a directory:
```bash
audio-metadata set -f music_directory -c cover.jpg
```

### Combining Operations

You can set multiple metadata fields in one command:
```bash
audio-metadata set -f song.mp3 -a "Album Name" -r "Artist Name" -t "Song Title" -c cover.jpg
```

### Notes

- The tool creates backups of your files before modifying them
- Backups are stored in `/tmp/audio-metadata-{timestamp}/`
- You can safely delete the backup directory after verifying the changes
- If you haven't installed the binary system-wide, you can run it from the build directory:
  ```bash
  ./target/release/audio-metadata set -f song.mp3 -a "Album Name"
  ```

## Supported Formats

- FLAC (using metaflac)
- MP3 (using id3v2)

## License

This project is licensed under the MIT License - see the LICENSE file for details.