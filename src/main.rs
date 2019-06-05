mod util;

use std::fs;
use std::io;

use rodio::Sink;

use tui::{Frame, Terminal};
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::widgets::{Widget, Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style};

use crate::util::event::{Event, Events};
use crate::util::App;

/*fn draw_queue_tab(f: &mut Frame<TermionBackend>, app: &App, area: Rect) {
    //
}

fn draw_library_tab(f: &mut Frame<TermionBackend>, app: &App, area: Rect) {
    //
}

fn draw_search_tab(f: &mut Frame<TermionBackend>, app: &App, area: Rect) {
    //
}

fn draw_browse_tab(f: &mut Frame<TermionBackend>, app: &App, area: Rect) {
    //
}*/

fn main() -> Result<(), failure::Error> {

    // Create the sink for the audio output device
    let sink = Sink::new(&rodio::default_output_device()
                         .expect("Error: No output device available."));

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
                        Constraint::Percentage(90),
                        Constraint::Percentage(5)
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
            Block::default()
                .title("now playing")
                .borders(Borders::ALL)
                .render(&mut f, chunks[2]);
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
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                Key::Char('s') => {
                    // Turn on shuffle
                },
                Key::Char('r') => {
                    // Turn on repeat
                },
                Key::Char('>') => {
                    // Skip to next song
                },
                Key::Char('<') => {
                    // Skip to previous song
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
