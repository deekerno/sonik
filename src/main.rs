pub mod application;
pub mod storage;
pub mod ui;
mod util;

use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};
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
use crate::storage::database::*;
use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    let matches = App::new("sonik")
        .version("0.9")
        .author("Alexander Decurnou. <ad@alx.xyz>")
        .about("A console music player")
        .arg(
            Arg::with_name("music_location")
                .short("d")
                .long("database_creation")
                .value_name("FOLDER")
                .help("Create the database using files from this location")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rebuild")
                .short("r")
                .long("rebuild")
                .help("Rebuild the library using location from config file"),
        )
        .get_matches();

    println!("Loading configuration...");

    let config = match matches.value_of("music_location") {
        Some(ml) => Config::new(ml).expect("Could not get or create configuration"),
        _ => Config::get_config().expect("Could not get or create configuration"),
    };

    println!("Loading database...");

    let (artists, stats) = if !Path::new(&config.database_path).exists() {
        create_and_load_database(&config).expect("Could not create database")
    } else if matches.is_present("rebuild") {
        rebuild_database(&config).expect("Could not rebuild database")
    } else {
        load_database(&config).expect("Could not load database")
    };

    let engine_group =
        create_fuzzy_searcher(&artists).expect("Could not create artist fuzzy search");

    // Create the sink for the audio output device
    let device = rodio::default_output_device().expect("No audio output device found");

    // Create the notification channel for empty
    // audio sink, pause/play events, and the track transfer channel
    let (btx, brx) = channel::bounded(0);
    let (ptx, prx) = channel::bounded(0);
    let (ttx, trx) = channel::bounded(0);

    // Keypress event handler, spins a thread
    let ui_events = Events::new();

    // Create structs to be managed on different threads
    let mut ui = UI::new(&artists, brx, ttx, ptx, engine_group, stats);
    let mut audio = Audio::new(device, trx, btx, prx);

    // All audio-related bits are sent to their own thread
    thread::spawn(move || {
        loop {
            // Alert the UI thread that there is no song playing
            if audio.sink.empty() {
                if let Ok(()) = audio.btx.send_timeout(true, Duration::from_millis(250)) {}
            } else if let Ok(()) = audio.btx.send_timeout(false, Duration::from_millis(250)) {
            }

            // If the UI thread semds a track from the queue,
            // receive it and send it to the sink
            if let Ok(track) = audio.trx.try_recv() {
                audio.play(track)
            }

            // Listen for a play/pause event
            match audio.prx.try_recv() {
                Ok(true) => audio.pause_play(),
                Ok(false) => audio.stop(),
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
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(91),
                        Constraint::Percentage(4),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            Block::default()
                .style(Style::default().bg(Color::Black))
                .render(&mut f, size);
            ui::screens::draw_top_bar(&mut f, &ui, chunks[0]);
            match ui.tabs.index {
                0 => ui::screens::draw_queue(&mut f, &ui, chunks[1]),
                1 => ui::screens::draw_library(&mut f, &ui, chunks[1]),
                2 => ui::screens::draw_search(&mut f, &ui, chunks[1]),
                _ => {}
            }
            ui::screens::draw_stats(&mut f, &ui, chunks[2]);
        })?;

        // Capture keypresses
        if let Event::Input(input) = ui_events.next()? {
            match input {
                Key::Char('p') => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push('p');
                    } else {
                        ui.pause_play();
                    }
                }
                Key::Esc => {
                    // Clear buffer so command line prompt is shown correctly
                    terminal.clear()?;
                    break;
                }
                Key::Char('s') => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push('s');
                    } else {
                        // Shuffle queue in place
                        ui.queue.shuffle();
                    }
                }
                Key::Char('r') => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push('r');
                    } else {
                        // Turn on repeat
                    }
                }
                Key::Char('>') => {
                    // Skip to next song
                    ui.play_from_queue();
                }
                Key::Char(' ') => {
                    if ui.tabs.index == 2 {
                        if !ui.search_results.is_empty() {
                            ui.add_to_queue();
                        } else {
                            ui.search_input.push(' ');
                        }
                    } else {
                        // Add track to queue
                        ui.add_to_queue();
                    }
                }
                Key::Char('c') => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push('c');
                    } else {
                        // Clear the queue
                        ui.clear_queue();
                    }
                }
                Key::Char('n') => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push('n');
                    } else {
                        // Add track to front of queue
                        ui.add_to_front();
                    }
                }
                Key::Char('1') => ui.tabs.index = 0,
                Key::Char('2') => ui.tabs.index = 1,
                Key::Char('3') => ui.tabs.index = 2,
                Key::Up => {
                    if ui.tabs.index == 1 {
                        ui.lib_cols.on_up();
                    } else if ui.tabs.index == 2 {
                        ui.on_up_search();
                    }
                }
                Key::Down => {
                    if ui.tabs.index == 1 {
                        ui.lib_cols.on_down();
                    } else if ui.tabs.index == 2 {
                        ui.on_down_search();
                    }
                }
                Key::Left => {
                    if ui.tabs.index == 1 {
                        ui.lib_cols.switch_left();
                    }
                }
                Key::Right => {
                    if ui.tabs.index == 1 {
                        ui.lib_cols.switch_right();
                    }
                }
                Key::Char('\n') => {
                    ui.on_enter();
                }
                Key::Char(c) => {
                    if ui.tabs.index == 2 {
                        ui.search_input.push(c);
                    }
                }
                Key::Backspace => {
                    if ui.tabs.index == 2 {
                        ui.search_input.pop();
                    }
                }
                _ => {}
            }
        }

        // Check for notifications that there is no audio being played
        if let Ok(true) = ui.rx.recv_timeout(Duration::from_millis(250)) {
            if ui.queue.is_empty() {
                ui.blank_now_play();
            } else {
                ui.play_from_queue();
            }
        }
    }
    Ok(())
}
