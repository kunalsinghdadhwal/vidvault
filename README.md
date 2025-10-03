# VidVault

A high-performance steganographic tool for encoding arbitrary binary data into video format, enabling compression-resistant data transmission and storage. VidVault converts files into video streams that can be uploaded to video-sharing platforms or transmitted through channels that may apply lossy compression.

## Overview

VidVault implements a robust video-based data encoding system that transforms binary data into visual patterns, allowing data to survive video compression algorithms. The system supports bidirectional conversion, enabling both embedding (data to video) and extraction (video to data) operations.

## Architecture

The system consists of three primary operations:

- **Embed**: Encodes binary data from files into AVI video format using configurable encoding parameters
- **Download**: Retrieves video files from URLs using yt-dlp integration
- **Dislodge**: Extracts and reconstructs original binary data from encoded videos

## Technical Specifications

### Encoding Modes

**Binary Mode**
- Direct byte-to-pixel encoding
- Higher redundancy and compression resistance
- Recommended for critical data transmission

**Colored Mode**
- Bit-level encoding with RGB channels
- Higher data density per frame
- Optimized for storage efficiency

### Preset Configurations

**MaxEfficiency**
- Resolution: 256x144
- Block Size: 1px
- Frame Rate: 10 fps
- Mode: Colored
- Use Case: Rapid encoding with minimal file size

**Optimal**
- Resolution: 1280x720
- Block Size: 2px
- Frame Rate: 10 fps
- Mode: Colored
- Use Case: Balanced performance and reliability

**Paranoid**
- Resolution: 1280x720
- Block Size: 4px
- Frame Rate: 10 fps
- Mode: Binary
- Use Case: Maximum compression resistance

## Installation

### Prerequisites

- Rust 2024 Edition or later
- OpenCV 4.x
- libclang (for OpenCV bindings)

### Build from Source

```bash
git clone <repository-url>
cd vidvault
cargo build --release
```

The compiled binary will be available at `target/release/vidvault`.

## Usage

### Interactive Mode

Launch without arguments for guided operation:

```bash
vidvault
```

### Command Line Interface

**Embed a File**

```bash
vidvault embed --in-path <input-file> --preset optimal
```

**Custom Encoding Parameters**

```bash
vidvault embed --in-path data.zip \
  --mode colored \
  --block-size 2 \
  --fps 10 \
  --resolution 720p \
  --threads 8
```

**Download Video**

```bash
vidvault download --url <video-url>
```

**Extract Data**

```bash
vidvault dislodge --in-path encoded.avi --out-path recovered.zip
```

## Configuration Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `--in-path` | String | Input file path for embedding or extraction |
| `--preset` | Enum | Predefined configuration (optimal, paranoid, maxefficiency) |
| `--mode` | Enum | Encoding mode (colored, binary) |
| `--block-size` | Integer | Pixel block size for encoding |
| `--threads` | Integer | Number of parallel processing threads |
| `--fps` | Integer | Output video frame rate |
| `--resolution` | String | Output resolution (144p, 240p, 360p, 480p, 720p) |

## Dependencies

- **OpenCV**: Video processing and frame manipulation
- **Tokio**: Asynchronous runtime for I/O operations
- **youtube_dl**: Video download functionality
- **Clap**: Command-line argument parsing
- **Inquire**: Interactive terminal prompts

## Performance Considerations

- Multi-threaded encoding leverages available CPU cores
- Block size directly impacts compression resistance and encoding speed
- Frame rate affects file size and temporal redundancy
- Binary mode provides approximately 2x redundancy compared to colored mode

## Output Format

Encoded videos are generated in AVI container format with uncompressed frames to preserve data integrity. The output file `output.avi` is created in the current working directory.

## Data Integrity

The system encodes file metadata and implements pixel-level data mapping to ensure accurate reconstruction. When using appropriate presets, the encoded data can survive multiple generations of lossy compression.

## Development Status

This project is in active development. The core encoding and decoding functionality is operational, with ongoing improvements to compression resistance algorithms and performance optimization.
