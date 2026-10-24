mod library;
mod player;

use std::{collections::BTreeMap, io, path::PathBuf, time::Duration};

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
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
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

// Playback state is independent of the currently browsed album.
struct PlayingTrack {
    artist: String,
    album: String,
    index: usize,
}

struct App {
    view: View,
    selected: usize,
    artist_selected: usize,
    album_selected: usize,
    now_playing: Option<String>,
    playing_track: Option<PlayingTrack>,
    repeat_mode: RepeatMode,
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
            repeat_mode: RepeatMode::Off,
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
            View::Songs(_, _) => {}
        }
    }

    fn back(&mut self) {
        match &self.view {
            View::Artists => {}
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

    let mut audio =
        player::AudioPlayer::new().map_err(|error| io::Error::other(error.to_string()))?;

    loop {
        // Handle track completion independently of the library browser.
        if app.playing_track.is_some() && audio.is_finished() {
            let playing = app.playing_track.take().expect("checked above");
            let tracks = music
                .get(&playing.artist)
                .and_then(|albums| albums.get(&playing.album));

            let next_index = match app.repeat_mode {
                RepeatMode::Song => Some(playing.index),
                RepeatMode::Album => tracks.and_then(|tracks| {
                    if tracks.is_empty() {
                        None
                    } else {
                        Some((playing.index + 1) % tracks.len())
                    }
                }),
                RepeatMode::Off => Some(playing.index + 1),
            };

            let next_song =
                next_index.and_then(|index| tracks.and_then(|tracks| tracks.get(index)).copied());

            if let (Some(index), Some(song)) = (next_index, next_song) {
                start_track(&mut app, &mut audio, song, index);
            } else {
                app.now_playing = None;
                app.status = "Album finished".into();
            }
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
                .style(
                    Style::default()
                        .fg(CYAN)
                        .add_modifier(Modifier::BOLD),
                )
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
                    let prefix = if i == app.selected {
                        "▸ "
                    } else {
                        "  "
                    };

                    let style = if i == app.selected {
                        Style::default()
                            .fg(CYAN)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(TEXT)
                    };

                    ListItem::new(format!("{prefix}{name}"))
                        .style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title(format!(" {heading} "))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(PURPLE)),
            );

            let mut list_state = ListState::default()
                .with_selected(
                    (!entries.is_empty()).then_some(app.selected),
                );

            frame.render_stateful_widget(
                list,
                columns[0],
                &mut list_state,
            );

            // Player information.
            let album_count: usize =
                music.values().map(|albums| albums.len()).sum();

            let playing = app
                .now_playing
                .as_deref()
                .unwrap_or("Nothing playing");

            let player_sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(8),
                    Constraint::Length(3),
                ])
                .split(columns[1]);

            // Prioritize the title when vertical space is limited. Wrapped titles
            // need room; statistics are secondary and can disappear on short screens.
            let mut info = format!(
                "Repeat: {}\nStatus: {}\n\nNOW PLAYING\n{}",
                app.repeat_mode.label(),
                playback_status,
                playing,
            );

            if player_sections[0].height >= 15 {
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
                        .title(format!(
                            " {} / {} ",
                            elapsed_label,
                            duration_label
                        ))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                )
                .gauge_style(
                    Style::default()
                        .fg(CYAN)
                        .bg(Color::Rgb(41, 46, 66)),
                )
                .ratio(progress)
                .label("");

            frame.render_widget(
                progress_bar,
                player_sections[1],
            );

            // Footer.
            let footer = Paragraph::new(
                " ↑↓ Navigate  → Open  ← Back  Enter Play  Space Pause  F +10s  R -10s  T Repeat  S Stop  Q Quit ",
            )
            .style(Style::default().fg(MUTED))
            .block(
                Block::default().borders(Borders::ALL),
            );

            frame.render_widget(footer, sections[2]);
        })?;

        // Redraw every 100 milliseconds.
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
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
                    View::Songs(artist, album) => {
                        let index = app.selected;
                        if let Some(song) = music[artist][album].get(index) {
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

                KeyCode::Char('t') => {
                    app.repeat_mode = app.repeat_mode.next();
                }

                KeyCode::Char('s') => {
                    audio.stop();
                    app.status = "Stopped".into();
                    app.now_playing = None;
                    app.playing_track = None;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
