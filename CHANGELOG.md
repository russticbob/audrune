# Changelog

All notable changes to AUDRUNE will be documented in this file.

AUDRUNE follows [Semantic Versioning](https://semver.org/). Pre-release versions are marked as beta.

## [0.7.3-beta] - 2026-10-26

### Added
- Command-line argument parsing using `clap`.
- `-h`, `--help` to display command-line usage.
- `-V`, `--version` to display the installed application version.
- `-i`, `--import <FILE>` to import local M3U and M3U8 playlists.
- Automatic playlist naming based on the imported filename.
- Playlist importing without launching the interactive player.

### Changed
- Updated the Arch Linux package to version `0.7.3-1`.
- Updated Rust dependencies and lockfile.

### Verified
- Successful CLI help and version output.
- Successful import of an M3U8 playlist containing a local audio file.
- Successful installation using the Arch Linux `PKGBUILD`.

---

## Earlier Releases

Earlier beta releases predate this changelog. Their changes can be reviewed in the Git commit history and release tags.

[0.
