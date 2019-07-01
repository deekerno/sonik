mod util;
pub mod application;
pub mod database;
pub mod ui;

use std::io;
use std::path::Path;
use std::thread;

//use log::*;
//use simplelog::*;
use rodio::Sink;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};
use tui::widgets::{Widget, Block, Borders, Tabs};

use crate::application::config::Config;
use crate::application::state::App;
use crate::database::database::{create_and_load_database, load_database};
use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {

    // Load the configuration for the program 
    // and attempt to load the database
    println!("Loading configuration...");
    let config = Config::get_config().expect("Could not get or create configuration");

    let artists;

    if !Path::new(&config.database_path).exists() {
        println!("Creating database...");
        artists = create_and_load_database(Path::new(&config.music_folder), Path::new(&config.database_path)).expect("Could not create database"); 
    } else {
        println!("Loading database...");
        artists = load_database(Path::new(&config.database_path)).expect("Could not load database");
    }

    // Create the sink for the audio output device
    let device = rodio::default_output_device().expect("No audio output device found");

    let events = Events::new();
    let mut app = App::new("sonik", &artists, &device);

    //debug - println!("Number of artists in database: {}", &app.database.len());

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

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
        })?;

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
                    /*app.updating_status = true;
                    thread::spawn(|| {
                        artists = create_and_load_database(
                            Path::new(&config.music_folder), 
                            Path::new(&config.database_path))
                            .expect("Could not create database"); 
                    });
                    app.updating_status = false;*/
                },
                Key::Char('>') => {
                    // Skip to next song
                },
                Key::Char('<') => {
                    // Skip to previous song
                },
                Key::Char(' ') => {
                    // Add track to queue
                    app.add_to_queue();
                },
                Key::Char('c') => {
                    // Clear the queue  
                },
                Key::Char('n') => {
                    // Add track to front of queue
                },
                Key::Char('1') => app.tabs.index = 0,
                Key::Char('2') => app.tabs.index = 1,
                Key::Char('3') => app.tabs.index = 2,
                Key::Char('4') => app.tabs.index = 3,
                Key::Up => app.lib_cols.on_up(),
                Key::Down => app.lib_cols.on_down(),
                Key::Left => app.lib_cols.switch_left(),
                Key::Right => app.lib_cols.switch_right(),
                Key::Char('\n') => app.play_now(),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
