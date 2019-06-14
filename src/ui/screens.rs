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
        Text::styled("title, album, artist\nyear_less, year_greater", Style::default().fg(Color::Yellow))
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
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("tabs"))
        .titles(&app.tabs.titles)
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("       ")
        .render(f, chunks[0]);
    Block::default()
        .borders(Borders::ALL)
        .title("now playing")
        .render(f, chunks[1]);
    Block::default()
        .borders(Borders::ALL)
        .title("status")
        .render(f, chunks[2]);
}
