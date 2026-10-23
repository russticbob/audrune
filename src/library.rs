use std::{
    fs,
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
    scan_directory(root, &mut songs);
    songs
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

        // Audio properties are independent of artist/title tags.
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
