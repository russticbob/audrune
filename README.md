# AUDRUNE

**Your terminal. Your music.**

A modern, keyboard-driven music player for Linux, written in Rust.

AUDRUNE brings local music libraries, playlist management, playback queues, and a responsive terminal interface together in one lightweight application.

Built for Linux users who want control over their music without leaving the terminal.

**Latest version: v0.7.3-beta** · **License: GPLv3** · **Platform: Linux**

[Download AUDRUNE](https://github.com/russticbob/audrune/releases) · [Report a Bug](https://github.com/russticbob/audrune/issues) · [Source Code](https://github.com/russticbob/audrune)

---

## Overview

AUDRUNE is a free and open-source terminal music player designed around three principles: **music ownership, simplicity, and performance**.

Streaming services offer convenience, but locally owned music provides independence, permanence, and control.

AUDRUNE starts with a great local listening experience and aims to eventually unify personal music collections, digital music purchases, and supported streaming services.

The application is currently in public beta, with functional local music playback, library navigation, playlist support, and queue management.

## Features

- **Terminal-native interface:** A responsive TUI built with Ratatui.
- **Local library browsing:** Discover music and navigate artists, albums, and tracks.
- **Metadata support:** Read embedded audio metadata and organize tracks accordingly.
- **Audio playback:** Play, pause, resume, stop, skip, and return to previous tracks.
- **Playback queue:** Manage upcoming tracks with automatic advancement.
- **Shuffle and repeat:** Choose how your music plays.
- **Playlists:** Create and manage `.m3u8` playlists.
- **Now Playing:** View the current track, duration, and playback progress.
- **Tokyo Night-inspired styling:** A clean, colorful interface designed for modern terminals.
- **Keyboard-first controls:** Navigate and control playback without a mouse.
- **Included demo track:** Try AUDRUNE immediately, even without a music library.

### Supported Audio Formats

AUDRUNE currently recognizes these file extensions:

| Format | Extension |
|---|---|
| MP3 | `.mp3` |
| FLAC | `.flac` |
| Ogg Vorbis | `.ogg` |
| Opus | `.opus` |
| MPEG-4 Audio | `.m4a` |
| WAV | `.wav` |

Playback compatibility depends on the audio backend and supported codecs. Full compatibility testing is ongoing.

## Screenshots

Screenshots of the library browser and Now Playing interface are coming soon.

## Installation

**Current release: v0.7.1-beta**

AUDRUNE currently targets **Linux x86-64**.

Download release packages from the [AUDRUNE GitHub Releases](https://github.com/russticbob/audrune/releases) page.

### Arch Linux and Arch-Based Distributions

For Arch Linux, CachyOS, EndeavourOS, and other compatible distributions, download:

`audrune-0.7.1-1-x86_64.pkg.tar.zst`

Open a terminal in your download directory:

```bash
cd ~/Downloads
sudo pacman -U ./audrune-0.7.1-1-x86_64.pkg.tar.zst
```

Launch AUDRUNE:

```bash
audrune
```

Pacman handles the package's declared runtime dependencies.

### Other Linux Distributions

For Ubuntu, Debian, Linux Mint, Fedora, openSUSE, and other compatible Linux distributions, use the portable Linux archive.

Download:

`audrune-v0.7.1-beta-linux-x86_64.tar.gz`

Extract it:

```bash
cd ~/Downloads
tar -xzf audrune-v0.7.1-beta-linux-x86_64.tar.gz
```

Enter the extracted directory:

```bash
cd audrune-v0.7.1-beta-linux-x86_64
```

Launch:

```bash
./audrune
```

The portable archive includes the application, bundled demo audio, license, and documentation.

**Compatibility note:** The portable executable is not fully static. Compatibility depends on your distribution's system libraries and audio configuration.

#### Optional: System-Wide Portable Installation

To make the portable version available from any directory, run the following commands from inside the extracted archive directory:

```bash
sudo mkdir -p /opt/audrune
sudo cp -a ./audrune ./assets ./LICENSE ./README.md /opt/audrune/
sudo ln -s /opt/audrune/audrune /usr/local/bin/audrune
```

Launch it with:

```bash
audrune
```

This is a manual installation, not managed by your distribution's package manager. Ensure `/usr/local/bin/audrune` does not already exist before creating the symbolic link.

### Build from Source

AUDRUNE can also be compiled directly from its source code.

**Requirements:**

- Linux
- A recent Rust toolchain and Cargo
- ALSA development libraries
- A working audio output device
- A terminal emulator with Unicode support; true-color support recommended

#### Arch Linux / CachyOS

```bash
sudo pacman -S --needed base-devel rust cargo alsa-lib pkgconf
```

#### Ubuntu / Debian / Linux Mint

```bash
sudo apt update
sudo apt install build-essential cargo rustc libasound2-dev pkg-config
```

#### Fedora

```bash
sudo dnf install gcc rust cargo alsa-lib-devel pkgconf-pkg-config
```

**Note:** Some distribution-provided Rust versions may be too old for AUDRUNE's dependencies. If necessary, install a newer toolchain using [rustup](https://rustup.rs/).

#### Clone and Compile

```bash
git clone https://github.com/russticbob/audrune.git
cd audrune
cargo build --release --locked
```

Run:

```bash
./target/release/audrune
```

Alternatively:

```bash
cargo run --release --locked
```

### Upcoming Distribution Packages

Additional native Linux packages are planned.

| Package | Intended distributions | Status |
|---|---|---|
| Pacman (`.pkg.tar.zst`) | Arch Linux, CachyOS, EndeavourOS, compatible distributions | v0.7.1-beta |
| Portable (`.tar.gz`) | Compatible Linux x86-64 distributions | v0.7.1-beta |
| Debian (`.deb`) | Debian, Ubuntu, Linux Mint | **Coming soon** |
| RPM (`.rpm`) | Fedora, openSUSE, RHEL-compatible distributions | **Coming soon** |

Native DEB and RPM packages are planned to simplify installation, dependency management, upgrades, and removal.

### Verify Your Download

When a matching checksum file is available, verify your download before extracting it:

```bash
sha256sum -c audrune-v0.7.1-beta-linux-x86_64.tar.gz.sha256
```

A successful verification reports `OK`.

## Getting Started

AUDRUNE currently scans your `~/Music` directory for local audio files.

1. Place your music files inside `~/Music`, organized however you prefer.
2. Launch AUDRUNE.
3. Browse artists, albums, and tracks using the keyboard.
4. Select a track and press `Enter` to begin playback.

If you don't have any local music yet, AUDRUNE includes a demo track you can use to test playback.

Configurable library directories and improved library management are planned for future releases.

## Included Demo Track

### Yes! It Can Djent!

**Artist:** AUDRUNE  
**Album:** AUDRUNE ORIGINALS  
**Format:** FLAC

Every official v0.7.1-beta distribution package includes a demo track to demonstrate AUDRUNE's playback capabilities.

Because even a terminal music player deserves some ridiculously heavy guitar riffs.

The track was created using Suno and is distributed as a separate creative asset. It is not automatically covered by the software's GPLv3 license.

See `assets/demo/README.md` for details about the demo recording.

## Keyboard Controls

AUDRUNE is designed for keyboard-first navigation and playback. All controls are available directly from the terminal interface.

### Navigation

| Key | Action |
|---|---|
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `→` / `l` | Open selected artist or album |
| `←` / `h` / `Backspace` | Return to previous view |
| `Enter` | Open selection or play the selected track |
| `?` | Show keyboard help |
| `q` / `Esc` | Quit AUDRUNE |

### Playback

| Key | Action |
|---|---|
| `Space` | Pause or resume playback |
| `s` | Stop playback |
| `n` | Skip to the next track |
| `f` | Seek forward |
| `r` | Seek backward |
| `t` | Cycle repeat modes |
| `u` | Cycle shuffle modes |
| `x` | Play a random song immediately |

### Queue Management

| Key | Action |
|---|---|
| `a` | Add the highlighted song to the playback queue |
| `n` | Play the next queued track, or advance album playback |
| `c` | Clear the playback queue |

### Playlist Management

| Key | Action |
|---|---|
| `p` | Toggle between the playlist browser and music library |
| `m` | Create a new playlist |
| `Enter` | Open a playlist or play the selected playlist track |
| `g` | Add the highlighted library song to the active playlist |
| `z` | Remove the highlighted track from a saved playlist |
| `e` | Empty a saved playlist |
| `d` | Delete a saved playlist |

Removing a track from a playlist, emptying a playlist, or deleting a playlist **does not delete the underlying music files**.

Destructive playlist operations require confirmation.

### Help and Dialogs

| Key | Action |
|---|---|
| `?` | Open or close keyboard help |
| `Esc` | Close help or cancel playlist naming |
| `Enter` | Confirm a new playlist name |
| `Backspace` | Delete a character while entering a playlist name |

Keyboard shortcuts are lowercase and may evolve during beta development.

## Development Roadmap

AUDRUNE is under active development. The roadmap below describes completed milestones and planned functionality.

Priorities and implementation details may change as the project matures.

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
- [x] Responsive terminal layouts
- [ ] Real-time audio spectrum visualization
- [ ] Embedded album artwork
- [ ] Automatic retrieval of missing album artwork
- [ ] Local artwork caching
- [ ] Branded fallback artwork
- [ ] Customizable themes

### Phase 3 — Library Management

- [x] Playlist creation and management
- [x] Shuffle and repeat modes
- [ ] Configurable music directories
- [ ] Library search and filtering
- [ ] Improved metadata handling
- [ ] Library refresh and change detection

### Phase 4 — Digital Music Stores

- [ ] Integration with supported digital music retailers
- [ ] Album discovery and retailer matching
- [ ] Browser-based purchase workflows
- [ ] Authorized downloads of purchased music
- [ ] Automatic organization of downloaded albums
- [ ] Integration of purchases into the local library

Bandcamp and other digital retailers offering downloadable music are being considered for future integration.

### Phase 5 — Streaming Services

- [ ] Modular streaming provider architecture
- [ ] Investigation of YouTube Music integration
- [ ] Investigation of Deezer integration
- [ ] Unified browsing of local and supported streaming libraries
- [ ] Clear distinction between locally stored and streaming-only music

Streaming functionality will depend on available provider interfaces, licensing requirements, and applicable terms of service.

### Phase 6 — Distribution and Packaging

- [x] Rust release builds
- [x] Portable Linux x86-64 distribution
- [x] Arch Linux package
- [ ] Debian / Ubuntu DEB package
- [ ] Fedora / openSUSE RPM package
- [ ] Automated release builds
- [ ] Additional architecture support
- [ ] Expanded distribution compatibility testing

## Technology Stack

AUDRUNE is written in Rust and uses the following technologies:

| Component | Technology |
|---|---|
| Programming language | Rust |
| Terminal user interface | Ratatui |
| Terminal input and control | Crossterm |
| Audio metadata | Lofty |
| Audio playback | Rodio |
| Linux audio backend | ALSA |

The project is designed to support future expansion while keeping its core playback functionality straightforward and maintainable.

## Design Philosophy

### Your Music, Your Control

Locally owned music offers independence from subscription services, licensing changes, and disappearing catalogs.

AUDRUNE prioritizes your personal music collection while exploring ways to integrate additional music sources in the future.

### Terminal Native

No browser window. No Electron runtime. No unnecessary graphical desktop environment.

Just a responsive terminal interface built for keyboard-driven use.

### Performance

AUDRUNE aims to remain lightweight, fast, and efficient, even as its library and playback capabilities expand.

### Customization

The interface uses a Tokyo Night-inspired visual style, with future plans for customizable themes, layouts, and keyboard shortcuts.

### Open Source

AUDRUNE is developed openly under the GNU General Public License version 3.

The goal is to build software that users can inspect, modify, share, and improve.

## Contributing

Contributions are welcome.

Whether you're a Rust developer, Linux enthusiast, music collector, or terminal customization fanatic, there are ways to help improve AUDRUNE.

You can contribute by reporting bugs, proposing features, testing compatibility across distributions, improving documentation, or submitting code.

Please use [GitHub Issues](https://github.com/russticbob/audrune/issues) for bug reports and feature requests.

Additional contribution guidelines will be introduced as the project matures.

## Project Status

**AUDRUNE v0.7.1-beta — Public Beta**

AUDRUNE provides functional local music browsing, playback, queue management, playlists, and shuffle/repeat controls.

The project is not yet feature-complete or production-ready. Users may encounter bugs, incomplete features, or changes between beta releases.

The immediate focus is reliable playback, a polished terminal interface, and broader Linux distribution support.

## License

AUDRUNE's software source code is licensed under the **GNU General Public License version 3 (GPLv3)**.

See [LICENSE](LICENSE) for the full license text.

The bundled demo recording is a separate creative asset and is not automatically licensed under GPLv3. See [assets/demo/README.md](assets/demo/README.md) for its accompanying notice.

---

**AUDRUNE — Your terminal. Your music.**

*Made for Linux. Powered by Rust. Best enjoyed loud.*
