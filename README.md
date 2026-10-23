# HEVY

**A modern, terminal-based music player for Linux.**

HEVY is an open-source music player written in Rust, designed to provide a fast, intuitive, and customizable listening experience directly from the terminal.

The project aims to bring local music libraries, streaming services, and digital music purchases together in a single application, giving users greater flexibility in how they discover, manage, and enjoy music.

HEVY is currently in early development, with functional local library browsing and audio playback.

## Overview

Music listening has increasingly shifted toward subscription-based streaming platforms. While these services offer convenience and extensive catalogs, managing a personal music collection often requires separate applications and workflows.

HEVY is being developed to provide a unified experience that supports both streaming and locally owned music.

The long-term goal is to allow users to discover music through supported streaming services, purchase albums from digital retailers, and automatically organize downloaded files within their local music libraries.

Throughout development, HEVY will prioritize performance, usability, customization, and user control.

## Current Features

- Terminal-based interface built with Ratatui
- Local music library scanning and discovery
- Audio metadata extraction
- Artist, album, and track navigation
- Audio playback using Rodio
- Play, pause, resume, and stop controls
- Keyboard-driven navigation
- Track ordering based on embedded metadata
- Tokyo Night-inspired interface
- Support for scanning common audio file formats

### Audio Formats

HEVY currently recognizes the following file extensions:

- MP3
- FLAC
- OGG
- OPUS
- M4A
- WAV

Playback compatibility depends on the codecs supported by the audio backend. Full compatibility testing is ongoing.

## Installation

HEVY is currently available as source code. Prebuilt packages and distribution-specific installation methods may be introduced in future releases.

### Requirements

- Linux
- Rust and Cargo
- A supported audio output device
- A terminal emulator with Unicode and true-color support recommended

### Build from Source

Clone the repository:

```bash
git clone https://github.com/russticbob/hevy.git
cd hevy
```

Build the application:

```bash
cargo build --release
```

Run HEVY:

```bash
cargo run --release
```

HEVY currently scans the user's `~/Music` directory. Support for configurable library locations is planned.

## Keyboard Controls

| Key | Action |
|---|---|
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `→` / `l` | Open selected artist or album |
| `←` / `h` | Return to previous view |
| `Enter` | Open selection or play a track |
| `Space` | Pause or resume playback |
| `s` | Stop playback |
| `q` / `Esc` | Exit the application |

## Development Roadmap

HEVY is under active development. The following roadmap outlines planned functionality and may evolve as the project progresses.

### Phase 1 — Core Playback

- [x] Terminal user interface
- [x] Local music library discovery
- [x] Audio metadata extraction
- [x] Artist and album browsing
- [x] Basic audio playback controls
- [ ] Playback progress and duration tracking
- [ ] Automatic track advancement
- [ ] Previous and next track
