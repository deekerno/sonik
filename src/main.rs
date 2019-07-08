pub mod application;
pub mod database;
pub mod ui;
mod util;

use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

//use log::*;
//use simplelog::*;
use crossbeam_channel as channel;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Widget};
use tui::Terminal;

use crate::application::config::Config;
use crate::application::state::{Audio, UI};
use crate::database::database::{create_and_load_database, load_database};
use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    // Load the configuration for the program
    // and attempt to load the database
    println!("Loading configuration...");
    let config = Config::get_config().expect("Could not get or create configuration");

    let artists;

    // Get or create the database
    if !Path::new(&config.database_path).exists() {
        println!("Creating database...");
        artists = create_and_load_database(
            Path::new(&config.music_folder),
            Path::new(&config.database_path),
        )
        .expect("Could not create database");
    } else {
        println!("Loading database...");
        artists = load_database(Path::new(&config.database_path)).expect("Could not load database");
    }

    // Create the sink for the audio output device
    let device = rodio::default_output_device().expect("No audio output device found");

    // Create the notification channel for empty
    // audio sink and the track transfer channel
    let (btx, brx) = channel::bounded(0);
    let (ptx, prx) = channel::bounded(0);
    let (ttx, trx) = channel::bounded(0);

    // Keypress event handler, spins a thread
    let ui_events = Events::new();

    // Create structs to be managed on different threads
    let mut ui = UI::new("sonik", &artists, brx, ttx, ptx);
    let mut audio = Audio::new(device, trx, btx, prx);

    //debug - println!("Number of artists in database: {}", &app.database.len());

    // All audio-related bits are sent to their own thread
    thread::spawn(move || {
        loop {
            // Alert the UI thread that there is no song playing
            if audio.sink.empty() {
                audio.btx.send_timeout(true, Duration::from_millis(250));
            } else {
                audio.btx.send_timeout(false, Duration::from_millis(250));
            }

            // If the UI thread semds a track from the queue,
            // receive it and send it to the sink
            match audio.trx.try_recv() {
                Ok(track) => audio.play(track),
                _ => {}
            }

            // Listen for a play/pause event
            match audio.prx.try_recv() {
                Ok(true) => audio.pause_play(),
                _ => {}
            }
        }
    });

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
                .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
                .split(f.size());
            Block::default()
                .style(Style::default().bg(Color::Black))
                .render(&mut f, size);
            ui::screens::draw_top_bar(&mut f, &ui, chunks[0]);
            match ui.tabs.index {
                0 => ui::screens::draw_queue(&mut f, &ui, chunks[1]),
                1 => ui::screens::draw_library(&mut f, &ui, chunks[1]),
                2 => ui::screens::draw_search(&mut f, &ui, chunks[1]),
                3 => ui::screens::draw_browse(&mut f, &ui, chunks[1]),
                _ => {}
            }
        })?;

        // Capture keypresses
        match ui_events.next()? {
            Event::Input(input) => match input {
                Key::Char('p') => {
                    ui.pause_play();
                }
                Key::Char('q') => {
                    // Clear buffer so command line prompt is shown correctly
                    terminal.clear()?;
                    break;
                }
                Key::Char('s') => {
                    // Shuffle queue in place
                    ui.queue.shuffle();
                }
                Key::Char('r') => {
                    // Turn on repeat
                }
                Key::Char('u') => {
                    /*app.updating_status = true;
                    thread::spawn(|| {
                        artists = create_and_load_database(
                            Path::new(&config.music_folder),
                            Path::new(&config.database_path))
                            .expect("Could not create database");
                    });
                    app.updating_status = false;*/
                }
                Key::Char('>') => {
                    // Skip to next song
                    ui.play_from_queue();
                }
                Key::Char(' ') => {
                    // Add track to queue
                    ui.add_to_queue();
                }
                Key::Char('c') => {
                    // Clear the queue
                    ui.queue.clear();
                }
                Key::Char('n') => {
                    // Add track to front of queue
                    ui.add_to_front();
                }
                Key::Char('1') => ui.tabs.index = 0,
                Key::Char('2') => ui.tabs.index = 1,
                Key::Char('3') => ui.tabs.index = 2,
                Key::Char('4') => ui.tabs.index = 3,
                Key::Up => ui.lib_cols.on_up(),
                Key::Down => ui.lib_cols.on_down(),
                Key::Left => ui.lib_cols.switch_left(),
                Key::Right => ui.lib_cols.switch_right(),
                Key::Char('\n') => ui.play_now(),
                _ => {}
            },
            _ => {}
        }

        match ui.rx.recv_timeout(Duration::from_millis(250)) {
            Ok(true) => {
                if ui.queue.is_empty() {
                    ui.blank_now_play();
                } else {
                    ui.play_from_queue();
                }
            }
            _ => {}
        }
    }
    Ok(())
}
