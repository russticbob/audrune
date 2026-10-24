# AUDRUNE

**A modern, terminal-based music player for Linux.**

AUDRUNE is an open-source music player written in Rust, designed to provide a fast, intuitive, and customizable listening experience directly from the terminal.

The project aims to bring local music libraries, streaming services, and digital music purchases together in a single application, giving users greater flexibility in how they discover, manage, and enjoy music.

AUDRUNE is currently in early development, with functional local library browsing and audio playback.

## Overview

Music listening has increasingly shifted toward subscription-based streaming platforms. While these services offer convenience and extensive catalogs, managing a personal music collection often requires separate applications and workflows.

AUDRUNE is being developed to provide a unified experience that supports both streaming and locally owned music.

The long-term goal is to allow users to discover music through supported streaming services, purchase albums from digital retailers, and automatically organize downloaded files within their local music libraries.

Throughout development, AUDRUNE will prioritize performance, usability, customization, and user control.

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

AUDRUNE currently recognizes the following file extensions:

- MP3
- FLAC
- OGG
- OPUS
- M4A
- WAV

Playback compatibility depends on the codecs supported by the audio backend. Full compatibility testing is ongoing.

## Installation

AUDRUNE is currently available as source code. Prebuilt packages and distribution-specific installation methods may be introduced in future releases.

### Requirements

- Linux
- Rust and Cargo
- A supported audio output device
- A terminal emulator with Unicode and true-color support recommended

### Build from Source

Clone the repository:

```bash
git clone https://github.com/russticbob/audrune.git
cd audrune
```

Build the application:

```bash
cargo build --release
```

Run AUDRUNE:

```bash
cargo run --release
```

AUDRUNE currently scans the user's `~/Music` directory. Support for configurable library locations is planned.

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

AUDRUNE is under active development. The following roadmap outlines planned functionality and may evolve as the project progresses.

### Phase 1 — Core Playback

- [x] Terminal user interface
- [x] Local music library discovery
- [x] Audio metadata extraction
- [x] Artist and album browsing
- [x] Basic audio playback controls
- [x] Playback progress and duration tracking
- [x] Automatic track advancement
- [x] Previous and next track controls
- [x] Playback queue management

### Phase 2 — Interface and Visualization

- [x] Dedicated Now Playing interface
- [ ] Real-time audio spectrum visualization
- [ ] Embedded album artwork
- [ ] Automatic retrieval of missing album artwork
- [ ] Local artwork caching
- [ ] Branded fallback artwork
- [ ] Customizable themes
- [x] Responsive terminal layouts

### Phase 3 — Library Management

- [ ] Configurable music directories
- [ ] Library search and filtering
- [x] Playlist creation and management
- [x] Shuffle and repeat modes
- [ ] Improved metadata handling
- [ ] Library refresh and change detection

### Phase 4 — Digital Music Stores

- [ ] Integration with supported digital music retailers
- [ ] Album discovery and retailer matching
- [ ] Browser-based purchase workflows
- [ ] Authorized downloads of purchased music
- [ ] Automatic organization of downloaded albums
- [ ] Integration of purchases into the local library

Bandcamp and other retailers offering downloadable music are being considered for future integration.

### Phase 5 — Streaming Services

- [ ] Modular streaming provider architecture
- [ ] Investigation of YouTube Music integration
- [ ] Investigation of Deezer integration
- [ ] Unified browsing of local and supported streaming libraries
- [ ] Clear distinction between locally stored and streaming-only music

Streaming functionality will depend on the availability, licensing requirements, and capabilities of supported provider interfaces.

## Technology Stack

AUDRUNE is developed in Rust using the following libraries:

| Component | Technology |
|---|---|
| Programming language | Rust |
| Terminal interface | Ratatui |
| Terminal input and control | Crossterm |
| Audio metadata | Lofty |
| Audio playback | Rodio |

The application is designed around a modular architecture to support future expansion without unnecessarily complicating its core functionality.

## Design Principles

### Performance

AUDRUNE aims to remain lightweight and responsive, with efficient library management and minimal unnecessary resource consumption.

### Accessibility and Usability

A keyboard-first interface provides direct access to essential playback and navigation functions. Future development will focus on clear layouts, configurable controls, and consistent interaction patterns.

### Music Ownership

Streaming services provide convenient access to music, while locally owned files offer greater independence and long-term availability.

AUDRUNE aims to support both approaches, making it easier to maintain a permanent music collection without sacrificing the convenience of streaming.

### Customization

The interface is being designed with customization in mind, including future support for themes, configurable layouts, and personalized keyboard shortcuts.

### Open Development

AUDRUNE is developed openly, with an emphasis on maintainable code, clear documentation, and community participation.

## Contributing

AUDRUNE is an early-stage project, and contributions are welcome.

Developers and users are encouraged to submit bug reports, feature suggestions, and improvement proposals through GitHub Issues.

As the project matures, additional contribution guidelines and development documentation will be introduced.

## Project Status

**Early development / Pre-release**

AUDRUNE currently provides functional local music browsing and basic audio playback on Linux.

The application is not yet considered feature-complete or production-ready. Features described in the roadmap are planned and should not be interpreted as currently available.

## License

A software license will be selected and published before the project's initial public release.

---

