use std::io::{self, Write};

use chrono::Local;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Tabs, Text, Widget};
use tui::Frame;

use crate::application::state::UI;
use crate::ui::widgets::RecordList;

// Yeah, I know this isn't elegant, but hey it works
pub fn artist_color(app: &UI) -> Style {
    let color;

    if app.lib_cols.current_active == 0 {
        color = Style::default().fg(Color::Rgb(255, 255, 0))
    } else {
        color = Style::default().fg(Color::Rgb(173, 176, 73))
    }

    color
}

pub fn album_color(app: &UI) -> Style {
    let color;

    if app.lib_cols.current_active == 1 {
        color = Style::default().fg(Color::Rgb(255, 255, 0))
    } else {
        color = Style::default().fg(Color::Rgb(173, 176, 73))
    }

    color
}

pub fn track_color(app: &UI) -> Style {
    let color;

    if app.lib_cols.current_active == 2 {
        color = Style::default().fg(Color::Rgb(255, 255, 0))
    } else {
        color = Style::default().fg(Color::Rgb(173, 176, 73))
    }

    color
}
pub fn draw_queue<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area);

    let artists = app.queue.tracks.iter().map(|track| {
        Text::styled(
            track.artist.to_string(),
            Style::default().fg(Color::LightGreen),
        )
    });

    let titles = app.queue.tracks.iter().map(|track| {
        Text::styled(
            track.title.to_string(),
            Style::default().fg(Color::LightBlue),
        )
    });

    let albums = app.queue.tracks.iter().map(|track| {
        Text::styled(
            track.album.to_string(),
            Style::default().fg(Color::LightRed),
        )
    });

    Block::default()
        .borders(Borders::ALL)
        .title("up next")
        .render(f, area);

    List::new(titles)
        .block(Block::default())
        .render(f, chunks[0]);

    List::new(artists)
        .block(Block::default())
        .render(f, chunks[1]);

    List::new(albums)
        .block(Block::default())
        .render(f, chunks[2]);
}

pub fn draw_library<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .direction(Direction::Horizontal)
        .split(area);

    // This will be the artist block
    RecordList::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(artist_color(&app)),
        )
        .items(&app.lib_cols.artists.items)
        .select(Some(app.lib_cols.artists.selected))
        .style(Style::default().fg(Color::White))
        .highlight_style(artist_color(&app).modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .render(f, chunks[0]);

    // This will be the albums of that artist
    RecordList::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(album_color(&app)),
        )
        .items(&app.lib_cols.albums.items)
        .select(Some(app.lib_cols.albums.selected))
        .style(Style::default().fg(Color::White))
        .highlight_style(album_color(&app).modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .render(f, chunks[1]);

    // This will be the songs of that album of that artist
    RecordList::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(track_color(&app)),
        )
        .items(&app.lib_cols.tracks.items)
        .select(Some(app.lib_cols.tracks.selected))
        .style(Style::default().fg(Color::White))
        .highlight_style(track_color(&app).modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .render(f, chunks[2]);
}

pub fn draw_search<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Ratio(1, 10), Constraint::Ratio(9, 10)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    draw_search_input(f, app, chunks[0]);
    draw_search_results(f, app, chunks[1]);
}

fn draw_search_input<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(area);

    let text = [
        Text::styled("Available Terms:\n", Style::default().fg(Color::Yellow)),
        Text::styled("\ntitle, album, artist", Style::default().fg(Color::Yellow)),
    ];

    // Enclosing border
    Block::default()
        .borders(Borders::ALL)
        .title("query")
        .render(f, area);

    // Term explanations
    Paragraph::new(text.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, chunks[1]);

    // Input box
    Paragraph::new([Text::raw(&app.search_input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL))
        .render(f, chunks[2]);

    io::stdout().flush().ok();
}

fn draw_search_results<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    RecordList::default()
        .block(Block::default().borders(Borders::ALL).title("results"))
        .items(&app.search_results)
        .select(Some(app.search_select))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 255, 0))
                .modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .render(f, chunks[0]);
}

pub fn draw_top_bar<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
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
        .divider("              ")
        .render(f, chunks[0]);

    draw_now_playing(f, chunks[1], app);

    draw_status(f, chunks[2]);
}

fn draw_now_playing<B>(f: &mut Frame<B>, area: Rect, app: &UI)
where
    B: Backend,
{
    let track_info = [
        Text::styled(
            &app.now_playing.title,
            Style::default().fg(Color::LightBlue),
        ),
        Text::raw(" - "),
        Text::styled(
            &app.now_playing.artist,
            Style::default().fg(Color::LightGreen),
        ),
        Text::raw(" - "),
        Text::styled(&app.now_playing.album, Style::default().fg(Color::LightRed)),
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

fn draw_status<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw(Local::now().date().format("%A, %B %d, %Y").to_string()),
        Text::raw(" | "),
        Text::raw(Local::now().time().format("%H:%M:%S").to_string()),
    ];

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

pub fn draw_stats<B>(f: &mut Frame<B>, app: &UI, area: Rect)
where
    B: Backend,
{
    let hours = app.queue.total_time / 3_600_000;
    let mins = (app.queue.total_time - (3_600_000 * hours)) / 60000;
    let secs = (app.queue.total_time - (3_600_000 * hours) - (mins * 60000)) / 1000;

    let total_hours = app.stats.total_time / 3_600_000;
    let total_mins = (app.stats.total_time - (3_600_000 * total_hours)) / 60000;
    let total_secs =
        (app.stats.total_time - (3_600_000 * total_hours) - (total_mins * 60000)) / 1000;

    let queue_info = [
        Text::raw(format!("remaining: {} tracks", app.queue.tracks.len())),
        Text::raw(" | "),
        Text::raw(format!(
            "{} hour(s), {} min(s), {} sec(s)",
            hours, mins, secs
        )),
    ];

    let library_info = [
        Text::raw(format!("{} artists", app.stats.artists)),
        Text::raw(" | "),
        Text::raw(format!("{} albums", app.stats.albums)),
        Text::raw(" | "),
        Text::raw(format!("{} tracks", app.stats.tracks)),
        Text::raw(" | "),
        Text::raw(format!(
            "{} hour(s), {} min(s), {} sec(s)",
            total_hours, total_mins, total_secs
        )),
    ];

    let search_blurb = [Text::raw("Enter search query")];

    let search_info = [Text::raw(format!("{} results", app.search_results.len()))];

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Vertical)
        .margin(1)
        .split(area);

    Block::default()
        .borders(Borders::ALL)
        .title("stats")
        .render(f, area);

    match app.tabs.index {
        0 => {
            Paragraph::new(queue_info.iter())
                .alignment(Alignment::Center)
                .render(f, chunks[0]);
        }
        1 => {
            Paragraph::new(library_info.iter())
                .alignment(Alignment::Center)
                .render(f, chunks[0]);
        }
        2 => {
            if !app.search_results.is_empty() {
                Paragraph::new(search_info.iter())
                    .alignment(Alignment::Center)
                    .render(f, chunks[0]);
            } else {
                Paragraph::new(search_blurb.iter())
                    .alignment(Alignment::Center)
                    .render(f, chunks[0]);
            }
        }
        _ => {}
    }
}
