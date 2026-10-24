mod library;
mod player;

use std::{
    collections::{BTreeMap, VecDeque},
    fs, io,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph},
};

// Tokyo Night color palette.
const CYAN: Color = Color::Rgb(125, 207, 255);
const PURPLE: Color = Color::Rgb(187, 154, 247);
const TEXT: Color = Color::Rgb(192, 202, 245);
const MUTED: Color = Color::Rgb(86, 95, 137);

#[derive(Clone)]
enum View {
    Artists,
    Albums(String),
    Songs(String, String),
    Playlists,
    PlaylistSongs(String),
}

type MusicLibrary<'a> = BTreeMap<String, BTreeMap<String, Vec<&'a library::Song>>>;

#[derive(Clone, Copy)]
enum RepeatMode {
    Off,
    Song,
    Album,
}

impl RepeatMode {
    fn next(self) -> Self {
        match self {
            Self::Off => Self::Song,
            Self::Song => Self::Album,
            Self::Album => Self::Off,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Song => "Song",
            Self::Album => "Album",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ShuffleMode {
    Off,
    Album,
    Library,
}

impl ShuffleMode {
    fn next(self) -> Self {
        match self {
            Self::Off => Self::Album,
            Self::Album => Self::Library,
            Self::Library => Self::Off,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Album => "Album",
            Self::Library => "Library",
        }
    }
}

// A tiny PRNG for track selection; no new Cargo dependencies.
fn random_index(limit: usize) -> usize {
    if limit <= 1 {
        return 0;
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let mut x = nanos ^ (nanos >> 32);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    (x as usize) % limit
}

// Playback state is independent of the currently browsed album.
#[derive(Clone)]
struct PlayingTrack {
    artist: String,
    album: String,
    index: usize,
}

#[derive(Clone, Copy)]
enum PlaylistAction {
    Clear,
    Delete,
    RemoveSong(usize),
}

struct App {
    view: View,
    selected: usize,
    artist_selected: usize,
    album_selected: usize,
    now_playing: Option<String>,
    playing_track: Option<PlayingTrack>,
    queue: VecDeque<PlayingTrack>,
    playlist_up_next: VecDeque<PlayingTrack>,
    playlist_playing: bool,
    playlists: BTreeMap<String, Vec<PathBuf>>,
    active_playlist: Option<String>,
    naming_playlist: Option<String>,
    pending_playlist_action: Option<(String, PlaylistAction)>,
    repeat_mode: RepeatMode,
    shuffle_mode: ShuffleMode,
    show_help: bool,
    status: String,
}

impl App {
    fn new() -> Self {
        Self {
            view: View::Artists,
            selected: 0,
            artist_selected: 0,
            album_selected: 0,
            now_playing: None,
            playing_track: None,
            queue: VecDeque::new(),
            playlist_up_next: VecDeque::new(),
            playlist_playing: false,
            playlists: BTreeMap::new(),
            active_playlist: None,
            naming_playlist: None,
            pending_playlist_action: None,
            repeat_mode: RepeatMode::Off,
            shuffle_mode: ShuffleMode::Off,
            show_help: false,
            status: "Ready to play".into(),
        }
    }

    fn up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn down(&mut self, count: usize) {
        if count > 0 {
            self.selected = (self.selected + 1).min(count - 1);
        }
    }

    fn enter(&mut self, entries: &[String]) {
        let Some(entry) = entries.get(self.selected) else {
            return;
        };

        match &self.view {
            View::Artists => {
                self.artist_selected = self.selected;
                self.view = View::Albums(entry.clone());
                self.selected = 0;
            }
            View::Albums(artist) => {
                self.album_selected = self.selected;
                self.view = View::Songs(artist.clone(), entry.clone());
                self.selected = 0;
            }
            View::Songs(_, _) | View::Playlists | View::PlaylistSongs(_) => {}
        }
    }

    fn back(&mut self) {
        match &self.view {
            View::Artists => {}
            View::Playlists => {
                self.view = View::Artists;
                self.selected = 0;
            }
            View::PlaylistSongs(_) => {
                self.view = View::Playlists;
                self.selected = 0;
            }
            View::Albums(_) => {
                self.view = View::Artists;
                self.selected = self.artist_selected;
            }
            View::Songs(artist, _) => {
                self.view = View::Albums(artist.clone());
                self.selected = self.album_selected;
            }
        }
    }
}

fn format_time(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn main() -> io::Result<()> {
    let music_dir =
        PathBuf::from(std::env::var_os("HOME").expect("HOME directory not found")).join("Music");

    let songs = library::scan_music(&music_dir);

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    if let Err(error) = execute!(stdout, EnterAlternateScreen) {
        disable_raw_mode()?;
        return Err(error);
    }

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(error) => {
            disable_raw_mode()?;
            let mut stdout = io::stdout();
            let _ = execute!(stdout, LeaveAlternateScreen);
            return Err(error);
        }
    };

    let result = run(&mut terminal, &songs);

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    result
}

// Start a track and remember where it belongs in its album.
fn start_track(app: &mut App, audio: &mut player::AudioPlayer, song: &library::Song, index: usize) {
    match audio.play(&song.path) {
        Ok(()) => {
            audio.set_duration(song.duration);
            app.now_playing = Some(format!("{}\n{}", song.artist, song.title));
            app.playing_track = Some(PlayingTrack {
                artist: song.artist.clone(),
                album: song.album.clone(),
                index,
            });
            app.status = "Playing".into();
        }
        Err(error) => {
            app.status = format!("Playback error: {error}");
            app.now_playing = None;
            app.playing_track = None;
        }
    }
}

// Resolve a queued entry against the already-sorted library.
fn queued_song<'a>(music: &'a MusicLibrary<'_>, entry: &PlayingTrack) -> Option<&'a library::Song> {
    music
        .get(&entry.artist)?
        .get(&entry.album)?
        .get(entry.index)
        .copied()
}

// Queue takes priority over album auto-next. Repeat Song applies only when
// the queue is empty, so manually queued songs are never starved.
fn advance(app: &mut App, audio: &mut player::AudioPlayer, music: &MusicLibrary<'_>) {
    while let Some(next) = app.queue.pop_front() {
        if let Some(song) = queued_song(music, &next) {
            start_track(app, audio, song, next.index);
            return;
        }
    }

    // Manual queue wins; playlist sequence comes next, then normal album/shuffle.
    while let Some(next) = app.playlist_up_next.pop_front() {
        if let Some(song) = queued_song(music, &next) {
            start_track(app, audio, song, next.index);
            return;
        }
    }

    if app.playlist_playing {
        app.playlist_playing = false;
        app.playing_track = None;
        app.now_playing = None;
        app.status = "Playlist finished".into();
        return;
    }

    let Some(playing) = app.playing_track.take() else {
        app.now_playing = None;
        app.status = "Queue finished".into();
        return;
    };
    let Some(tracks) = music
        .get(&playing.artist)
        .and_then(|a| a.get(&playing.album))
    else {
        app.now_playing = None;
        app.status = "Album unavailable".into();
        return;
    };
    // Explicitly queued tracks always win. Repeat Song wins over shuffle.
    if !matches!(app.repeat_mode, RepeatMode::Song) {
        let shuffled = match app.shuffle_mode {
            ShuffleMode::Off => None,
            ShuffleMode::Album => {
                if tracks.len() < 2 {
                    None
                } else {
                    let offset = 1 + random_index(tracks.len() - 1);
                    Some(PlayingTrack {
                        artist: playing.artist.clone(),
                        album: playing.album.clone(),
                        index: (playing.index + offset) % tracks.len(),
                    })
                }
            }
            ShuffleMode::Library => {
                let all: Vec<PlayingTrack> = music
                    .iter()
                    .flat_map(|(artist, albums)| {
                        albums.iter().flat_map(move |(album, tracks)| {
                            (0..tracks.len()).map(move |index| PlayingTrack {
                                artist: artist.clone(),
                                album: album.clone(),
                                index,
                            })
                        })
                    })
                    .filter(|candidate| {
                        candidate.artist != playing.artist
                            || candidate.album != playing.album
                            || candidate.index != playing.index
                    })
                    .collect();
                if all.is_empty() {
                    None
                } else {
                    Some(all[random_index(all.len())].clone())
                }
            }
        };
        if let Some(next) = shuffled {
            if let Some(song) = queued_song(music, &next) {
                start_track(app, audio, song, next.index);
                return;
            }
        }
    }
    let next_index = match app.repeat_mode {
        RepeatMode::Song => Some(playing.index),
        RepeatMode::Album if !tracks.is_empty() => Some((playing.index + 1) % tracks.len()),
        _ => Some(playing.index + 1),
    };
    if let Some((index, song)) = next_index.and_then(|i| tracks.get(i).map(|song| (i, *song))) {
        start_track(app, audio, song, index);
    } else {
        app.now_playing = None;
        app.status = "Album finished".into();
    }
}

// M3U8 playlists are portable UTF-8 files with absolute music paths.
fn playlist_directory() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
        .join("Music")
        .join("Playlists")
}

fn valid_playlist_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && !name.starts_with('.')
        && !name.contains(['/', '\\', '\n', '\r'])
        && !name.chars().any(|c| c.is_control())
}

fn read_playlists(dir: &Path) -> io::Result<BTreeMap<String, Vec<PathBuf>>> {
    let mut playlists = BTreeMap::new();
    fs::create_dir_all(dir)?;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|x| x.to_str()) != Some("m3u8") {
            continue;
        }
        let Some(name) = path.file_stem().and_then(|x| x.to_str()) else {
            continue;
        };
        let contents = fs::read_to_string(&path)?;
        let tracks = contents
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                let p = PathBuf::from(line);
                if p.is_absolute() { p } else { dir.join(p) }
            })
            .collect();
        playlists.insert(name.to_owned(), tracks);
    }
    Ok(playlists)
}

fn save_playlist(dir: &Path, name: &str, paths: &[PathBuf]) -> io::Result<()> {
    if !valid_playlist_name(name) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid playlist name",
        ));
    }
    fs::create_dir_all(dir)?;
    let mut contents = String::from("#EXTM3U\n");
    for path in paths {
        contents.push_str(&path.to_string_lossy());
        contents.push('\n');
    }
    fs::write(dir.join(format!("{name}.m3u8")), contents)
}

fn resolve_path(music: &MusicLibrary<'_>, path: &Path) -> Option<PlayingTrack> {
    for (artist, albums) in music {
        for (album, songs) in albums {
            for (index, song) in songs.iter().enumerate() {
                if song.path.as_path() == path {
                    return Some(PlayingTrack {
                        artist: artist.clone(),
                        album: album.clone(),
                        index,
                    });
                }
            }
        }
    }
    None
}

fn play_playlist_from(
    app: &mut App,
    audio: &mut player::AudioPlayer,
    music: &MusicLibrary<'_>,
    name: &str,
    from: usize,
) {
    let Some(paths) = app.playlists.get(name) else {
        return;
    };
    let tracks: Vec<PlayingTrack> = paths
        .iter()
        .skip(from)
        .filter_map(|path| resolve_path(music, path))
        .collect();
    let mut iter = tracks.into_iter();
    let Some(first) = iter.next() else {
        app.status = "Playlist has no available tracks".into();
        return;
    };
    app.playlist_up_next = iter.collect();
    app.playlist_playing = true;
    if let Some(song) = queued_song(music, &first) {
        start_track(app, audio, song, first.index);
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    songs: &[library::Song],
) -> io::Result<()> {
    // Organize music by artist, album, and track.
    let mut music: MusicLibrary = BTreeMap::new();

    for song in songs {
        music
            .entry(song.artist.clone())
            .or_default()
            .entry(song.album.clone())
            .or_default()
            .push(song);
    }

    for albums in music.values_mut() {
        for tracks in albums.values_mut() {
            tracks.sort_by(|a, b| {
                a.track
                    .unwrap_or(u32::MAX)
                    .cmp(&b.track.unwrap_or(u32::MAX))
                    .then_with(|| a.title.cmp(&b.title))
            });
        }
    }

    let mut app = App::new();
    let playlist_dir = playlist_directory();
    match read_playlists(&playlist_dir) {
        Ok(saved) => app.playlists = saved,
        Err(err) => app.status = format!("Playlist load error: {err}"),
    }

    let mut audio =
        player::AudioPlayer::new().map_err(|error| io::Error::other(error.to_string()))?;

    loop {
        // Advance automatically when the decoder finishes.
        if app.playing_track.is_some() && audio.is_finished() {
            advance(&mut app, &mut audio, &music);
        }

        // Build the current library view.
        let (heading, entries): (String, Vec<String>) = match &app.view {
            View::Artists => ("ARTISTS".into(), music.keys().cloned().collect()),
            View::Albums(artist) => (
                format!("ALBUMS / {artist}"),
                music[artist].keys().cloned().collect(),
            ),
            View::Songs(artist, album) => (
                format!("{artist} / {album}"),
                music[artist][album]
                    .iter()
                    .map(|song| format!("{:02}. {}", song.track.unwrap_or(0), song.title))
                    .collect(),
            ),
            View::Playlists => ("PLAYLISTS".into(), app.playlists.keys().cloned().collect()),
            View::PlaylistSongs(name) => (
                format!("PLAYLIST / {name}"),
                app.playlists
                    .get(name)
                    .into_iter()
                    .flatten()
                    .map(|path| {
                        resolve_path(&music, path)
                            .and_then(|track| queued_song(&music, &track))
                            .map(|song| format!("{} — {}", song.artist, song.title))
                            .unwrap_or_else(|| format!("[Missing] {}", path.display()))
                    })
                    .collect(),
            ),
        };

        // Playback information.
        let position = audio.position();
        let duration = audio.duration();

        let progress = duration
            .filter(|total| !total.is_zero())
            .map(|total| (position.as_secs_f64() / total.as_secs_f64()).clamp(0.0, 1.0))
            .unwrap_or(0.0);

        let elapsed_label = format_time(position);

        let duration_label = duration.map(format_time).unwrap_or_else(|| "--:--".into());

        let playback_status = if app.now_playing.is_none() {
            app.status.clone()
        } else if audio.is_finished() {
            "Finished".into()
        } else if audio.is_paused() {
            "Paused".into()
        } else {
            "Playing".into()
        };

        terminal.draw(|frame| {
            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            // Header.
            let header = Paragraph::new("◆ HEVY / LOCAL MUSIC")
                .style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                );

            frame.render_widget(header, sections[0]);

            // Stack panels on narrow terminals; keep side-by-side on wide ones.
            let stacked = sections[1].width < 110;
            let columns = if stacked {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
                    .split(sections[1])
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(sections[1])
            };

            // Library list.
            let items: Vec<ListItem> = entries
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let prefix = if i == app.selected { "▸ " } else { "  " };

                    let style = if i == app.selected {
                        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(TEXT)
                    };

                    ListItem::new(format!("{prefix}{name}")).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title(format!(" {heading} "))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(PURPLE)),
            );

            let mut list_state =
                ListState::default().with_selected((!entries.is_empty()).then_some(app.selected));

            frame.render_stateful_widget(list, columns[0], &mut list_state);

            // Player information.
            let album_count: usize = music.values().map(|albums| albums.len()).sum();

            let playing = app.now_playing.as_deref().unwrap_or("Nothing playing");

            let player_sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(8), Constraint::Length(3)])
                .split(columns[1]);

            // Prioritize the title when vertical space is limited. Wrapped titles
            // need room; statistics are secondary and can disappear on short screens.
            let mut info = format!(
                "Repeat: {}  |  Shuffle: {}\nStatus: {}\n\nNOW PLAYING\n{}",
                app.repeat_mode.label(),
                app.shuffle_mode.label(),
                playback_status,
                playing,
            );

            info.push_str(&format!("\n\nQUEUE: {} track(s)", app.queue.len()));
            for (index, queued) in app.queue.iter().take(3).enumerate() {
                if let Some(song) = queued_song(&music, queued) {
                    info.push_str(&format!(
                        "\n{}. {} — {}",
                        index + 1,
                        song.artist,
                        song.title
                    ));
                }
            }
            if let Some(name) = &app.active_playlist {
                info.push_str(&format!("\nACTIVE PLAYLIST: {name}"));
            }
            if !app.playlist_up_next.is_empty() {
                info.push_str(&format!(
                    "\nPLAYLIST UP NEXT: {}",
                    app.playlist_up_next.len()
                ));
            }
            if app.queue.len() > 3 {
                info.push_str(&format!("\n... and {} more", app.queue.len() - 3));
            }

            if player_sections[0].height >= 22 {
                info.push_str(&format!(
                    "\n\nArtists: {}\nAlbums: {}\nTracks: {}",
                    music.len(),
                    album_count,
                    songs.len(),
                ));
            }

            let details = Paragraph::new(info)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .style(Style::default().fg(TEXT))
                .block(
                    Block::default()
                        .title(" HEVY PLAYER ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                );

            frame.render_widget(details, player_sections[0]);

            // Live progress bar.
            let progress_bar = Gauge::default()
                .block(
                    Block::default()
                        .title(format!(" {} / {} ", elapsed_label, duration_label))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                )
                .gauge_style(Style::default().fg(CYAN).bg(Color::Rgb(41, 46, 66)))
                .ratio(progress)
                .label("");

            frame.render_widget(progress_bar, player_sections[1]);

            // One visible line inside the bordered footer (height = 3).
            // Choose text that fits the actual available inner width.
            let footer_width = sections[2].width.saturating_sub(2) as usize;
            let footer_options = [
                " ↑↓ Browse  Enter Play  X Random  A Queue  N Next  U Shuffle  T Repeat  ? Help ",
                " Enter Play  X Random  U Shuffle  ? Help ",
                " X Random  ? Help ",
                " ? Help ",
                "?",
            ];
            let footer_text = footer_options
                .iter()
                .find(|text| text.chars().count() <= footer_width)
                .copied()
                .unwrap_or("");
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(MUTED))
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(footer, sections[2]);

            if let Some(name) = &app.naming_playlist {
                let area = frame.area();
                let popup = ratatui::layout::Rect {
                    x: area.x + area.width.saturating_sub(58) / 2,
                    y: area.y + area.height.saturating_sub(5) / 2,
                    width: area.width.min(58),
                    height: area.height.min(5),
                };
                frame.render_widget(Clear, popup);
                let prompt = Paragraph::new(format!(
                    "Playlist name: {name}_\nEnter to save / Esc to cancel"
                ))
                .block(
                    Block::default()
                        .title(" NEW PLAYLIST ")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(CYAN));
                frame.render_widget(prompt, popup);
            }

            if let Some((name, action)) = &app.pending_playlist_action {
                let area = frame.area();
                let popup = ratatui::layout::Rect {
                    x: area.x + area.width.saturating_sub(64) / 2,
                    y: area.y + area.height.saturating_sub(5) / 2,
                    width: area.width.min(64),
                    height: area.height.min(5),
                };
                frame.render_widget(Clear, popup);
                let warning = match action {
                    PlaylistAction::Clear => {
                        format!("Empty playlist '{name}'?\nY: Confirm  N/Esc: Cancel")
                    }
                    PlaylistAction::Delete => format!(
                        "Delete playlist '{name}'? (Music is safe)\nY: Confirm  N/Esc: Cancel"
                    ),
                    PlaylistAction::RemoveSong(index) => format!(
                        "Remove song {} from playlist '{name}'?\nY: Confirm  N/Esc: Cancel",
                        index + 1
                    ),
                };
                let prompt = Paragraph::new(warning)
                    .wrap(ratatui::widgets::Wrap { trim: true })
                    .block(
                        Block::default()
                            .title(" CONFIRM PLAYLIST ACTION ")
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(CYAN));
                frame.render_widget(prompt, popup);
            }

            if app.show_help {
                let area = frame.area();
                let popup = ratatui::layout::Rect {
                    x: area.x + area.width.saturating_sub(66) / 2,
                    y: area.y + area.height.saturating_sub(24) / 2,
                    width: area.width.min(66),
                    height: area.height.min(26),
                };
                frame.render_widget(Clear, popup);
                let help = Paragraph::new(concat!(
                    "↑/↓ or J/K    Browse library\n",
                    "→/← or L/H    Open / Back\n",
                    "Enter         Play selected track\n",
                    "Space         Pause / Resume\n",
                    "F / R         Seek forward / backward 10 seconds\n",
                    "A             Add selected song to queue\n",
                    "N             Play next queued / album track\n",
                    "C             Clear queue\n",
                    "T             Repeat: Off / Song / Album\n",
                    "U             Shuffle: Off / Album / Library\n",
                    "X             Play random song from library\n",
                    "P             Toggle playlists / library\n",
                    "M             Create and name a playlist\n",
                    "G             Add selected song to active playlist\n",
                    "E             Empty selected playlist (confirm)\n",
                    "D             Delete selected playlist (confirm)\n",
                    "Z             Remove highlighted playlist song\n",
                    "Enter         Open / play playlist tracks\n",
                    "S             Stop playback\n",
                    "Q             Quit HEVY\n",
                    "? or Esc      Close help",
                ))
                .wrap(ratatui::widgets::Wrap { trim: false })
                .style(Style::default().fg(TEXT))
                .block(
                    Block::default()
                        .title(" HEVY HELP ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                );
                frame.render_widget(help, popup);
            }
        })?;

        // Redraw every 100 milliseconds.
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if let Some((name, action)) = app.pending_playlist_action.take() {
                if matches!(key.code, KeyCode::Char('y') | KeyCode::Char('Y')) {
                    match action {
                        PlaylistAction::Clear => match save_playlist(&playlist_dir, &name, &[]) {
                            Ok(()) => {
                                app.playlists.insert(name.clone(), Vec::new());
                                if matches!(&app.view, View::PlaylistSongs(current) if current == &name)
                                {
                                    app.selected = 0;
                                }
                                app.status = format!("Emptied playlist: {name}");
                            }
                            Err(err) => app.status = format!("Playlist clear error: {err}"),
                        },
                        PlaylistAction::RemoveSong(index) => {
                            if let Some(paths) = app.playlists.get(&name) {
                                if index < paths.len() {
                                    let mut updated = paths.clone();
                                    updated.remove(index);
                                    match save_playlist(&playlist_dir, &name, &updated) {
                                        Ok(()) => {
                                            let remaining = updated.len();
                                            app.playlists.insert(name.clone(), updated);
                                            app.selected = if remaining == 0 {
                                                0
                                            } else {
                                                app.selected.min(remaining - 1)
                                            };
                                            app.status = format!(
                                                "Removed song from {name} (music untouched)"
                                            );
                                        }
                                        Err(err) => {
                                            app.status = format!("Playlist save error: {err}")
                                        }
                                    }
                                } else {
                                    app.status = "Song no longer exists in playlist".into();
                                }
                            }
                        }
                        PlaylistAction::Delete => {
                            match fs::remove_file(playlist_dir.join(format!("{name}.m3u8"))) {
                                Ok(()) => {
                                    app.playlists.remove(&name);
                                    if app.active_playlist.as_deref() == Some(name.as_str()) {
                                        app.active_playlist = None;
                                    }
                                    if matches!(&app.view, View::PlaylistSongs(current) if current == &name)
                                    {
                                        app.view = View::Playlists;
                                    }
                                    app.selected = 0;
                                    app.status =
                                        format!("Deleted playlist: {name} (music untouched)");
                                }
                                Err(err) => app.status = format!("Playlist delete error: {err}"),
                            }
                        }
                    }
                } else {
                    app.status = "Playlist action canceled".into();
                }
                continue;
            }

            if let Some(name) = &mut app.naming_playlist {
                match key.code {
                    KeyCode::Esc => app.naming_playlist = None,
                    KeyCode::Backspace => {
                        name.pop();
                    }
                    KeyCode::Char(ch) if !ch.is_control() => {
                        name.push(ch);
                    }
                    KeyCode::Enter => {
                        let proposed = name.trim().to_owned();
                        app.naming_playlist = None;
                        if !valid_playlist_name(&proposed) {
                            app.status = "Invalid playlist name".into();
                        } else if app.playlists.contains_key(&proposed)
                            || playlist_dir.join(format!("{proposed}.m3u8")).exists()
                        {
                            app.status = "Playlist already exists".into();
                        } else {
                            match save_playlist(&playlist_dir, &proposed, &[]) {
                                Ok(()) => {
                                    app.playlists.insert(proposed.clone(), Vec::new());
                                    app.active_playlist = Some(proposed.clone());
                                    app.status = format!("Created playlist: {proposed}");
                                }
                                Err(err) => app.status = format!("Save error: {err}"),
                            }
                        }
                    }
                    _ => {}
                }
                continue;
            }

            if app.show_help {
                if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
                    app.show_help = false;
                }
                continue;
            }

            match key.code {
                KeyCode::Char('?') => app.show_help = true,
                KeyCode::Char('q') | KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Char('k') => {
                    app.up();
                }

                KeyCode::Down | KeyCode::Char('j') => {
                    app.down(entries.len());
                }

                KeyCode::Right | KeyCode::Char('l') => {
                    app.enter(&entries);
                }

                KeyCode::Left | KeyCode::Backspace | KeyCode::Char('h') => {
                    app.back();
                }

                KeyCode::Enter => match &app.view {
                    View::Playlists => {
                        if let Some(name) = entries.get(app.selected) {
                            app.active_playlist = Some(name.clone());
                            app.view = View::PlaylistSongs(name.clone());
                            app.selected = 0;
                        }
                    }
                    View::PlaylistSongs(name) => {
                        let name = name.clone();
                        let selected = app.selected;
                        play_playlist_from(&mut app, &mut audio, &music, &name, selected);
                    }
                    View::Songs(artist, album) => {
                        let index = app.selected;
                        if let Some(song) = music[artist][album].get(index) {
                            app.playlist_up_next.clear();
                            app.playlist_playing = false;
                            start_track(&mut app, &mut audio, song, index);
                        }
                    }
                    _ => app.enter(&entries),
                },

                KeyCode::Char(' ') => {
                    if app.now_playing.is_some() && !audio.is_finished() {
                        audio.toggle_pause();
                    }
                }

                KeyCode::Char('f') => {
                    if app.now_playing.is_some() && !audio.is_finished() {
                        if let Err(error) = audio.seek_forward() {
                            app.status = format!("Seek error: {error}");
                        }
                    }
                }

                KeyCode::Char('r') => {
                    if app.now_playing.is_some() && !audio.is_finished() {
                        if let Err(error) = audio.seek_backward() {
                            app.status = format!("Seek error: {error}");
                        }
                    }
                }

                // Add the selected song to the end of the queue.
                KeyCode::Char('p') => {
                    if matches!(app.view, View::Playlists | View::PlaylistSongs(_)) {
                        app.view = View::Artists;
                    } else {
                        app.view = View::Playlists;
                    }
                    app.selected = 0;
                }

                KeyCode::Char('m') => {
                    app.naming_playlist = Some(String::new());
                }

                // E empties the saved playlist, D deletes its .m3u8 file.
                // Both actions require confirmation; neither deletes music files.
                KeyCode::Char('e') | KeyCode::Char('d') => {
                    let target = match &app.view {
                        View::PlaylistSongs(name) => Some(name.clone()),
                        View::Playlists => entries.get(app.selected).cloned(),
                        _ => None,
                    };
                    if let Some(name) = target.filter(|n| app.playlists.contains_key(n)) {
                        let action = if key.code == KeyCode::Char('e') {
                            PlaylistAction::Clear
                        } else {
                            PlaylistAction::Delete
                        };
                        app.pending_playlist_action = Some((name, action));
                    } else {
                        app.status = "Select a playlist first (press P)".into();
                    }
                }

                // Remove only the highlighted entry from a saved playlist.
                // This does not delete the audio file or change the temporary queue.
                KeyCode::Char('z') => {
                    if let View::PlaylistSongs(name) = &app.view {
                        if app
                            .playlists
                            .get(name)
                            .is_some_and(|paths| app.selected < paths.len())
                        {
                            app.pending_playlist_action =
                                Some((name.clone(), PlaylistAction::RemoveSong(app.selected)));
                        } else {
                            app.status = "No playlist song selected".into();
                        }
                    } else {
                        app.status = "Open a playlist and highlight a song first".into();
                    }
                }

                KeyCode::Char('g') => {
                    if let (Some(name), View::Songs(artist, album)) =
                        (&app.active_playlist, &app.view)
                    {
                        if let Some(song) = music
                            .get(artist)
                            .and_then(|albums| albums.get(album))
                            .and_then(|songs| songs.get(app.selected))
                        {
                            let name = name.clone();
                            if let Some(paths) = app.playlists.get(&name) {
                                let mut updated = paths.clone();
                                updated.push(song.path.clone());
                                match save_playlist(&playlist_dir, &name, &updated) {
                                    Ok(()) => {
                                        app.playlists.insert(name.clone(), updated);
                                        app.status = format!("Added to {name}");
                                    }
                                    Err(err) => app.status = format!("Save error: {err}"),
                                }
                            }
                        }
                    } else {
                        app.status = "Open a song and select/create a playlist first".into();
                    }
                }

                KeyCode::Char('a') => {
                    if let View::Songs(artist, album) = &app.view {
                        if music
                            .get(artist)
                            .and_then(|albums| albums.get(album))
                            .and_then(|tracks| tracks.get(app.selected))
                            .is_some()
                        {
                            app.queue.push_back(PlayingTrack {
                                artist: artist.clone(),
                                album: album.clone(),
                                index: app.selected,
                            });
                            app.status = format!("Queued {} track(s)", app.queue.len());
                        }
                    }
                }

                // Skip to the next queued song, or continue album playback.
                KeyCode::Char('n') => {
                    if app.playing_track.is_some()
                        || !app.queue.is_empty()
                        || !app.playlist_up_next.is_empty()
                    {
                        advance(&mut app, &mut audio, &music);
                    }
                }

                KeyCode::Char('c') => {
                    app.queue.clear();
                    app.status = "Queue cleared".into();
                }

                KeyCode::Char('t') => {
                    app.repeat_mode = app.repeat_mode.next();
                }

                KeyCode::Char('u') => {
                    app.shuffle_mode = app.shuffle_mode.next();
                }

                // Start a random song immediately, without changing shuffle or queue.
                KeyCode::Char('x') => {
                    let all_tracks: Vec<PlayingTrack> = music
                        .iter()
                        .flat_map(|(artist, albums)| {
                            albums.iter().flat_map(move |(album, tracks)| {
                                (0..tracks.len()).map(move |index| PlayingTrack {
                                    artist: artist.clone(),
                                    album: album.clone(),
                                    index,
                                })
                            })
                        })
                        .collect();

                    if all_tracks.is_empty() {
                        app.status = "Music library is empty".into();
                    } else {
                        let selected = &all_tracks[random_index(all_tracks.len())];
                        if let Some(song) = queued_song(&music, selected) {
                            app.playlist_up_next.clear();
                            app.playlist_playing = false;
                            start_track(&mut app, &mut audio, song, selected.index);
                        }
                    }
                }

                KeyCode::Char('s') => {
                    audio.stop();
                    app.status = "Stopped".into();
                    app.now_playing = None;
                    app.playing_track = None;
                    app.playlist_up_next.clear();
                    app.playlist_playing = false;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
