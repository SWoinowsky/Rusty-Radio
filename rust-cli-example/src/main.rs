
use std::{io, sync::mpsc, thread, time::{Instant, Duration}};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crossterm::{terminal, event};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal, widgets, text, style, layout};

const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

enum MenuItem {
    Home,
    Pets,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Pets => 1,
        }
    }
}

fn main()  -> Result<(), Box<dyn std::error::Error>>  {
    terminal::enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200); // This is going to be the tick rate, how fast/frequently it checks for input
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let event::Event::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Pets", "Add", "Delete", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        layout::Constraint::Length(3),
                        layout::Constraint::Min(2),
                        layout::Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = widgets::Paragraph::new("pet-CLI 2020 - all rights reserved")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                        .borders(widgets::Borders::ALL)
                        .style(style::Style::default().fg(style::Color::White))
                        .title("Copyright")
                        .border_type(widgets::BorderType::Plain),
                );


            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    text::Spans::from(vec![
                        text::Span::styled(
                            first,
                            style::Style::default()
                                .fg(style::Color::Yellow)
                                .add_modifier(style::Modifier::UNDERLINED),
                        ),
                        text::Span::styled(rest, style::Style::default().fg(style::Color::White)),
                    ])
                })
                .collect();

            let tabs = widgets::Tabs::new(menu)
                .select(active_menu_item.into())
                .block(widgets::Block::default().title("Menu").borders(widgets::Borders::ALL))
                .style(style::Style::default().fg(style::Color::White))
                .highlight_style(style::Style::default().fg(style::Color::Yellow))
                .divider(text::Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);

            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                event::KeyCode::Char('q') => {
                    terminal::disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                event::KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                event::KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
