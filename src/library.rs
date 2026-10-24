use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use lofty::{
    file::{AudioFile, TaggedFileExt},
    prelude::Accessor,
    probe::Probe,
};

pub struct Song {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub track: Option<u32>,
    pub duration: Duration,
    pub path: PathBuf,
}

pub fn scan_music(root: &Path) -> Vec<Song> {
    let mut songs = Vec::new();

    // Scan the user's music library.
    scan_directory(root, &mut songs);

    // Also scan AUDRUNE's bundled demo music.
    if let Some(demo_dir) = find_demo_directory() {
        // Avoid scanning the same directory twice.
        if !demo_dir.starts_with(root) {
            scan_directory(&demo_dir, &mut songs);
        }
    }

    songs
}

fn find_demo_directory() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    // Development: launched from the AUDRUNE project directory.
    if let Ok(cwd) = env::current_dir() {
        candidates.push(cwd.join("assets/demo"));
    }

    // Installed application: demo music next to the executable.
    if let Ok(exe) = env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            candidates.push(bin_dir.join("assets/demo"));
            candidates.push(bin_dir.join("demo"));

            // Development: target/debug or target/release.
            if let Some(project_dir) = bin_dir.parent().and_then(|p| p.parent()) {
                candidates.push(project_dir.join("assets/demo"));
            }

            // Conventional Linux installation:
            // /usr/bin/audrune -> /usr/share/audrune/demo
            if let Some(prefix) = bin_dir.parent() {
                candidates.push(prefix.join("share/audrune/demo"));
            }
        }
    }

    candidates.into_iter().find(|path| path.is_dir())
}

fn scan_directory(dir: &Path, songs: &mut Vec<Song>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, songs);
            continue;
        }

        let supported = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                ["mp3", "flac", "ogg", "m4a", "opus", "wav"]
                    .iter()
                    .any(|format| ext.eq_ignore_ascii_case(format))
            });

        if !supported {
            continue;
        }

        let Ok(tagged) = Probe::open(&path).and_then(|p| p.read()) else {
            continue;
        };

        let duration = tagged.properties().duration();
        let tag = tagged.primary_tag().or_else(|| tagged.first_tag());

        let title = tag
            .and_then(|t| t.title())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            });

        let artist = tag
            .and_then(|t| t.artist())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown Artist".into());

        let album = tag
            .and_then(|t| t.album())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown Album".into());

        let track = tag.and_then(|t| t.track());

        songs.push(Song {
            artist,
            album,
            title,
            track,
            duration,
            path,
        });
    }
}