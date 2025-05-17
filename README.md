# Audio Metadata Tool

A command-line tool for managing audio file metadata and converting FLAC files to MP3.

## Features

- Set metadata (artist, album, title) for MP3 and FLAC files
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
```

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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.