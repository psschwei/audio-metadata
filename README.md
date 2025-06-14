# Audio Metadata Tool

A command-line tool for managing audio file metadata and converting FLAC files to MP3.

## Features

- Set metadata (artist, album, title, track number) for MP3 and FLAC files
- Infer track names from filenames (automatically removes track numbers and file extensions)
- Infer track numbers based on sorted order of files in a directory
- Manually set track numbers for files or directories
- Add cover art to audio files
- Convert FLAC files to MP3 with metadata preservation
- Process single files or entire directories
- Automatic backup of original files
- Supports x86_64 and ARM64 architectures on Linux and macOS

## Installation

### From Release

Download the latest release for your platform from the [Releases page](https://github.com/yourusername/audio-metadata/releases). The following binaries are available:

- Linux (x86_64): `audio-metadata-linux-x86_64`
- Linux (ARM64): `audio-metadata-linux-aarch64`
- macOS (x86_64): `audio-metadata-macos-x86_64`
- macOS (ARM64): `audio-metadata-macos-aarch64`
- Windows (x86_64): `audio-metadata-windows-x86_64.exe`

### Dependencies

The tool requires the following dependencies to be installed:

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get install ffmpeg id3v2 flac
```

#### macOS
```bash
brew install ffmpeg id3v2 flac
```

#### Windows
- Download and install [ffmpeg](https://ffmpeg.org/download.html)
- Download and install [id3v2](https://id3v2.sourceforge.net/)
- Download and install [flac](https://xiph.org/flac/download.html)

Make sure these tools are available in your system's PATH.

### From Source

```bash
git clone https://github.com/yourusername/audio-metadata.git
cd audio-metadata
cargo install --path .
```

## Usage

### Setting Metadata

```bash
# Set metadata for a single file
audio-metadata set -f song.mp3 -r "Artist Name" -a "Album Title" -t "Song Title"

# Set metadata for all files in a directory
audio-metadata set -f /path/to/music/dir -r "Artist Name" -a "Album Title"

# Add cover art
audio-metadata set -f song.mp3 -c cover.jpg

# Set track number for a single file
audio-metadata set -f song.mp3 -n 5

# Set track number for all files in a directory (all will be set to the same number)
audio-metadata set -f /path/to/music/dir -n 3

# Infer track names from filenames (removes track numbers and file extensions)
audio-metadata set -f /path/to/music/dir --infer-track

# Infer track numbers based on sorted order of files in directory
audio-metadata set -f /path/to/music/dir --infer-order

# Combine inferring track names with other metadata
audio-metadata set -f /path/to/music/dir -r "Artist Name" -a "Album Title" --infer-track

# Combine inferring track numbers with other metadata
audio-metadata set -f /path/to/music/dir -r "Artist Name" -a "Album Title" --infer-order

# Combine manual track number with other metadata
audio-metadata set -f song.mp3 -n 2 -r "Artist" -a "Album"
```

**Track Name Inference Examples:**
- `03 - This Song.mp3` → `This Song`
- `1 - Another Song.flac` → `Another Song`
- `01. Third Song.mp3` → `Third Song`
- `5. Fourth Song.flac` → `Fourth Song`
- `01_Fifth Song.mp3` → `Fifth Song`
- `10 Sixth Song.flac` → `Sixth Song`
- `Song Without Number.mp3` → `Song Without Number`

**Track Number Inference Examples:**
When using `--infer-order` on a directory containing:
- `03 - Song 1.mp3` → Track 1
- `04 - Song 2.mp3` → Track 2
- `01 - Song 3.flac` → Track 3
- `02 - Song 4.flac` → Track 4

The files are sorted alphabetically by filename, and track numbers are assigned sequentially starting from 1.

**Manual Track Number:**
- Using `-n`/`--track` sets the track number for a single file or all files in a directory (all will get the same number).
- If both `--infer-order` and `-n` are used, `--infer-order` takes precedence and assigns sequential track numbers.

### Converting FLAC to MP3

```bash
# Convert a single FLAC file to MP3
audio-metadata convert -f song.flac

# Convert with custom bitrate
audio-metadata convert -f song.flac -b 256

# Convert all FLAC files in a directory
audio-metadata convert -f /path/to/flac/dir

# Convert to a different directory
audio-metadata convert -f /path/to/flac/dir -o /path/to/output/dir
```

## Development

### Building from Source

```bash
git clone https://github.com/yourusername/audio-metadata.git
cd audio-metadata
cargo build --release
```

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.