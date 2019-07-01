use rodio::Source;
use rodio::{Device, Sink};
use std::fs::File;
use std::io::BufReader;

use crate::application::queue::SonikQueue;
use crate::database::record::{Album, Artist, Track};

// Tabs only need name and ordering information
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct ListState<I> {
    pub items: Vec<I>,
    pub selected: usize,
}

impl<I> ListState<I>
where
    I: std::clone::Clone,
{
    fn new(items: &Vec<I>) -> ListState<I> {
        ListState {
            items: items.to_vec(),
            selected: 0,
        }
    }

    fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }
}

pub struct LibraryCols {
    pub artists: ListState<Artist>,
    pub albums: ListState<Album>,
    pub tracks: ListState<Track>,
    pub current_active: usize,
}

impl LibraryCols {
    pub fn switch_left(&mut self) {
        if self.current_active > 0 {
            self.current_active -= 1;
        }
    }

    pub fn switch_right(&mut self) {
        if self.current_active < 2 {
            self.current_active += 1;
        }
    }

    pub fn on_up(&mut self) {
        // List states need to be refreshed when scrolling through each column
        match self.current_active {
            0 => {
                self.artists.select_previous();
                self.albums = ListState::new(&self.artists.items[self.artists.selected].albums);
                self.tracks = ListState::new(&self.albums.items[self.albums.selected].tracks);
            }
            1 => {
                self.albums.select_previous();
                self.tracks = ListState::new(&self.albums.items[self.albums.selected].tracks);
            }
            2 => self.tracks.select_previous(),
            _ => {}
        };
    }

    pub fn on_down(&mut self) {
        match self.current_active {
            0 => {
                self.artists.select_next();
                self.albums = ListState::new(&self.artists.items[self.artists.selected].albums);
                self.tracks = ListState::new(&self.albums.items[self.albums.selected].tracks);
            }
            1 => {
                self.albums.select_next();
                self.tracks = ListState::new(&self.albums.items[self.albums.selected].tracks);
            }
            2 => self.tracks.select_next(),
            _ => {}
        };
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub queue: SonikQueue,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub lib_cols: LibraryCols,
    pub now_playing: Track,
    pub device: &'a Device,
    pub sink: Sink,
    pub updating_status: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, database: &Vec<Artist>, device: &'a Device) -> App<'a> {
        // Generate initial list states
        let art_col = ListState::new(database);
        let al_col = ListState::new(&art_col.items[art_col.selected].albums);
        let tr_col = ListState::new(&al_col.items[al_col.selected].tracks);

        // Associate them all together
        let lib_cols = LibraryCols {
            artists: art_col,
            albums: al_col,
            tracks: tr_col,
            current_active: 0,
        };

        App {
            title,
            queue: SonikQueue::new(),
            should_quit: false,
            tabs: TabsState::new(vec!["queue", "library", "search", "browse"]),
            lib_cols: lib_cols,
            now_playing: Track::dummy(),
            device,
            sink: Sink::new(&device),
            updating_status: false,
        }
    }

    pub fn play_now(&mut self) {
        if self.lib_cols.current_active == 2 {
            self.sink = Sink::new(&self.device);
            let track = self.lib_cols.tracks.items[self.lib_cols.tracks.selected].clone();
            let file = File::open(&track.file_path).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);
            self.now_playing = track;
        } else if self.lib_cols.current_active == 1 {

        } else {

        }
    }

    pub fn add_to_queue(&mut self) {
        if self.lib_cols.current_active == 2 {
            let track = self.lib_cols.tracks.items[self.lib_cols.tracks.selected].clone();
            self.queue.add(track);
        } else if self.lib_cols.current_active == 1 {
            for t in &self.lib_cols.albums.items[self.lib_cols.albums.selected].tracks {
                self.queue.add(t.clone());
            }
        } else {
            for a in &self.lib_cols.artists.items[self.lib_cols.artists.selected].albums {
                for t in &a.tracks {
                    self.queue.add(t.clone());
                }
            }
        }
    }
}
