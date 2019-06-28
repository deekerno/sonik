pub mod event;

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
    pub active: bool,
}

impl<I> ListState<I> where I: std::clone::Clone {
    fn new(items: &Vec<I>) -> ListState<I> {
        ListState { items: items.to_vec(), selected: 0, active: false }
    }

    fn new_active(items: &Vec<I>) -> ListState<I> {
        ListState { items: items.to_vec(), selected: 0, active: true }
    }

    fn empty() -> ListState<I> {
        ListState { items: Vec::new(), selected: 0, active: false }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub queue: SonikQueue,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub artists_col: ListState<Artist>,
    pub album_col: ListState<Album>,
    pub track_col: ListState<Track>,
    pub now_playing: Track,
    pub updating_status: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, database: &Vec<Artist>) -> App<'a> {
        let art_col = ListState::new_active(database);
        let al_col = ListState::new(&art_col.items[art_col.selected].albums);
        let tr_col = ListState::new(&al_col.items[al_col.selected].tracks);

        App {
            title,
            queue: SonikQueue::new(),
            should_quit: false,
            tabs: TabsState::new(vec!["queue", "library", "search", "browse"]),
            artists_col: art_col,
            album_col: al_col,
            track_col: tr_col,
            now_playing: Track::dummy(),
            updating_status: false
        }
    }
}
