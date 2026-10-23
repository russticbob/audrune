mod library;

use std::{collections::BTreeMap, io, path::PathBuf};

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
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

// --------------------------------------------------
// HEVY COLORS — TOKYO NIGHT
// --------------------------------------------------

const CYAN: Color = Color::Rgb(125, 207, 255);
const PURPLE: Color = Color::Rgb(187, 154, 247);
const TEXT: Color = Color::Rgb(192, 202, 245);
const MUTED: Color = Color::Rgb(86, 95, 137);

// --------------------------------------------------
// LIBRARY NAVIGATION
// --------------------------------------------------

#[derive(Clone)]
enum View {
    Artists,
    Albums(String),
    Songs(String, String),
}

type MusicLibrary<'a> = BTreeMap<String, BTreeMap<String, Vec<&'a library::Song>>>;

struct App {
    view: View,
    selected: usize,
    artist_selected: usize,
    album_selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            view: View::Artists,
            selected: 0,
            artist_selected: 0,
            album_selected: 0,
        }
    }

    fn down(&mut self, count: usize) {
        if count > 0 {
            self.selected = (self.selected + 1).min(count - 1);
        }
    }

    fn up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
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

            View::Songs(_, _) => {
                // Playback will be implemented next.
            }
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

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() -> io::Result<()> {
    let music_dir =
        PathBuf::from(std::env::var_os("HOME").expect("HOME directory not found")).join("Music");

    let songs = library::scan_music(&music_dir);

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &songs);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

// --------------------------------------------------
// APPLICATION LOOP
// --------------------------------------------------

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    songs: &[library::Song],
) -> io::Result<()> {
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

    loop {
        let (heading, entries): (String, Vec<String>) = match &app.view {
            View::Artists => ("ARTISTS".to_string(), music.keys().cloned().collect()),

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

        terminal.draw(|frame| {
            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            // Header
            let header = Paragraph::new("◆ HEVY / LOCAL MUSIC")
                .style(Style::default().fg(CYAN).add_modifier(Modifier::BOLD))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(PURPLE)),
                );

            frame.render_widget(header, sections[0]);

            // Main columns
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
                .split(sections[1]);

            // Library entries
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

            // Information panel
            let album_count: usize = music.values().map(|a| a.len()).sum();

            let info = format!(
                "Artists: {}\nAlbums: {}\nTracks: {}\n\n\
                 Selected: {}\n\n\
                 Playback coming soon...",
                music.len(),
                album_count,
                songs.len(),
                entries
                    .get(app.selected)
                    .map(String::as_str)
                    .unwrap_or("None"),
            );

            let details = Paragraph::new(info).style(Style::default().fg(TEXT)).block(
                Block::default()
                    .title(" LIBRARY INFO ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(PURPLE)),
            );

            frame.render_widget(details, columns[1]);

            // Footer
            let footer = Paragraph::new(" ↑↓ Navigate  → Open  ← Back  Enter Select  Q Quit ")
                .style(Style::default().fg(MUTED))
                .block(Block::default().borders(Borders::ALL));

            frame.render_widget(footer, sections[2]);
        })?;

        // Keyboard input
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

                KeyCode::Right | KeyCode::Enter | KeyCode::Char('l') => {
                    app.enter(&entries);
                }

                KeyCode::Left | KeyCode::Backspace | KeyCode::Char('h') => {
                    app.back();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
