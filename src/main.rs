mod util;
pub mod database;
pub mod application;

use std::io;

use rusqlite::Connection;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::widgets::{Widget, Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};

use crate::util::event::{Event, Events};
use crate::util::App;
use crate::application::queue::SonikQueue;
use crate::application::config::Config;

fn main() -> Result<(), failure::Error> {

    // Load the configuration for the program 
    // and attempt to connect to database
    let config = Config::get_config().expect("Could not get or create configuration");
    let conn = Connection::open(config.database_path)?;
    database::database::create_database(&conn)?;

    // Create the sink for the audio output device
    let device = rodio::default_output_device().expect("No audio output device found");
    let queue = SonikQueue::new();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    
    let events = Events::new();
    let mut app = App::new("sonik");

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(95),
                    ].as_ref()
                )
                .split(f.size());
            Block::default()
                .style(Style::default().bg(Color::Black))
                .render(&mut f, size);
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("tabs"))
                .titles(&app.tabs.titles)
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);
            match app.tabs.index {
                0 => Block::default()
                    .title("queue")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                1 => Block::default()
                    .title("library")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                2 => Block::default()
                    .title("search")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                3 => Block::default()
                    .title("browse")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                _ => {}
            }
        });

        // Capture keypresses
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    // Clear buffer so command line prompt is shown correctly
                    terminal.clear()?;
                    break;
                },
                Key::Char('p') => {
                /*    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }*/
                },
                Key::Char('s') => {
                    // Turn on shuffle
                },
                Key::Char('r') => {
                    // Turn on repeat
                },
                Key::Char('u') => {
                    // update database
                }
                Key::Char('>') => {
                    // Skip to next song
                },
                Key::Char('<') => {
                    // Skip to previous song
                },
                Key::Char('a') => {
                    // Add track to queue
                },
                Key::Char('c') => {
                    // Clear the queue  
                },
                Key::Char('n') => {
                    // Add track to front of queue
                },
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
