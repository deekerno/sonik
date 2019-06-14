mod util;
pub mod database;
pub mod application;
pub mod ui;

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
use crate::application::config::Config;

fn main() -> Result<(), failure::Error> {

    // Load the configuration for the program 
    // and attempt to connect to database
    let config = Config::get_config().expect("Could not get or create configuration");
    let conn = Connection::open(config.database_path)?;
    database::database::create_database(&conn)?;

    // Create the sink for the audio output device
    let device = rodio::default_output_device().expect("No audio output device found");

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
            ui::screens::draw_top_bar(&mut f, &app, chunks[0]);
            match app.tabs.index {
                0 => ui::screens::draw_queue(&mut f, &app, chunks[1]),
                1 => ui::screens::draw_library(&mut f, &app, chunks[1]),
                2 => ui::screens::draw_search(&mut f, &app, chunks[1]),
                3 => ui::screens::draw_browse(&mut f, &app, chunks[1]),
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
                    database::database::update_database(&conn, &config.music_folder)?;
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
