use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let menu = ["Local Music", "YouTube Music", "Playlists", "Queue"];
    let mut selected = 0usize;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(area);

            let cyan = Color::Rgb(125, 207, 255);
            let purple = Color::Rgb(187, 154, 247);

            let header = Paragraph::new("◆ HEVY  /  v0.1.0")
                .style(Style::default().fg(cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(purple)));

            frame.render_widget(header, sections[0]);

            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(sections[1]);

            let items: Vec<ListItem> = menu
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let prefix = if i == selected { "▸ " } else { "  " };
                    let style = if i == selected {
                        Style::default().fg(cyan).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(192, 202, 245))
                    };

                    ListItem::new(format!("{prefix}{name}")).style(style)
                })
                .collect();

            let library = List::new(items)
                .block(Block::default().title(" LIBRARY ").borders(Borders::ALL).border_style(Style::default().fg(purple)));

            frame.render_widget(library, columns[0]);

            let now_playing = Paragraph::new("Nothing playing\n\nReady for music...")
                .style(Style::default().fg(Color::Rgb(192, 202, 245)))
                .block(Block::default().title(" NOW PLAYING ").borders(Borders::ALL).border_style(Style::default().fg(purple)));

            frame.render_widget(now_playing, columns[1]);

            let footer = Paragraph::new(" ↑↓ Navigate    Enter Select    Q Quit ")
                .style(Style::default().fg(cyan))
                .block(Block::default().borders(Borders::ALL));

            frame.render_widget(footer, sections[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up | KeyCode::Char('k') => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected = (selected + 1).min(menu.len() - 1);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
