use chrono::{Local, DateTime, NaiveTime};
use tui::backend::Backend;
use tui::Terminal;
use termion::raw::IntoRawMode;
use termion::event::Key;
use tui::widgets::{Widget, Block, Borders, Tabs, Text, List, Paragraph};
use tui::layout::{Layout, Constraint, Direction, Rect, Alignment};
use tui::style::{Color, Style};
use tui::Frame;

use crate::util::App;

pub fn draw_queue<B>(f: &mut Frame<B>, app: &App, area: Rect)
where 
    B: Backend,
{    
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);
    Block::default()
        .borders(Borders::ALL)
        .render(f, area);
    let songs = app.queue.tracks.iter().map(|track| {
        Text::raw(
            format!("{}\t\t{}\t\t{}", track.title, track.artist, track.album)    
        )
    });

    List::new(songs)
        .render(f, chunks[0]);
}

pub fn draw_library<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3)
            ].as_ref()
        )
        .direction(Direction::Horizontal)
        .split(area);
    
    // This will be the artist block
    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[0]);
    // This will be the albums of that artist
    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[1]);
    // This will be the songs of that album of that artist
    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[2]);
}

fn populate_artists() {
    //
}

fn populate_albums(artist: &String) {
    //
}

fn populate_tracks(artist: &String, album: &String) {
    //
}

pub fn draw_search<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1,10),
                Constraint::Ratio(9,10)
            ].as_ref()
        )
        .direction(Direction::Vertical)
        .split(area);

    draw_search_input(f, chunks[0]);
    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[1]);
    
}

fn draw_search_input<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Ratio(1,4),
                Constraint::Ratio(1,4),
                Constraint::Ratio(1,4),
                Constraint::Ratio(1,4),
            ].as_ref()
        )
        .split(area);

    let text = [
        Text::styled("Available Terms:\n", Style::default().fg(Color::Yellow)),
        Text::styled("title, album, artist\nyear_before, year_after", Style::default().fg(Color::Yellow))
    ];
    
    // Enclosing border
    Block::default()
        .borders(Borders::ALL)
        .render(f, area);

    // Term explanations
    Paragraph::new(text.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, chunks[1]);
    
    // Input box
    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[2]);

    // unhide terminal cursor here and set it to box start
}

fn draw_search_results<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    //
}

pub fn draw_browse<B>(f: &mut Frame<B>, app: &App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(100)
            ].as_ref()
        )
        .direction(Direction::Vertical)
        .split(area);

    Block::default()
        .borders(Borders::ALL)
        .render(f, chunks[0]);
}

pub fn draw_top_bar<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1,4),
                Constraint::Ratio(1,2),
                Constraint::Ratio(1,4)
            ].as_ref()
        )
        .direction(Direction::Horizontal)
        .split(area);
    
    // Draw tab explorer box
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("tabs"))
        .titles(&app.tabs.titles)
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("       ")
        .render(f, chunks[0]);

    draw_now_playing(f, chunks[1], app);

    draw_status(f, chunks[2], app);
}

fn draw_now_playing<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let track_info = [
        Text::styled(&app.now_playing.title, Style::default().fg(Color::LightBlue)),
        Text::raw(" - "),
        Text::styled(&app.now_playing.artist, Style::default().fg(Color::LightGreen)),
        Text::raw(" - "),
        Text::styled(&app.now_playing.album, Style::default().fg(Color::LightRed))
    ];

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Vertical)
        .margin(1)
        .split(area);

    Block::default()
        .borders(Borders::ALL)
        .title("now playing")
        .render(f, area);

    Paragraph::new(track_info.iter())
        .alignment(Alignment::Center)
        .render(f, chunks[0]);
}

fn draw_status<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    // This part doesn't work right now, but will soon
    let text = if app.updating_status {
        [Text::raw(""),Text::raw("Updating..."),Text::raw("")]
    } else {
        [
            Text::raw(
                Local::now()
                    .date()
                    .format("%A, %B %d, %Y")
                    .to_string()
            ),
            Text::raw(" | "),
            Text::raw(
                Local::now()
                    .time()
                    .format("%H:%M:%S")
                    .to_string()
            )
        ]
    };

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Vertical)
        .margin(1)
        .split(area);

    Block::default()
        .borders(Borders::ALL)
        .title("status")
        .render(f, area);

    Paragraph::new(text.iter())
        .alignment(Alignment::Center)
        .render(f, chunks[0]);
}
