pub mod event;

use crate::application::queue::SonikQueue;
use crate::database::record::{Artist, Track};


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

pub struct App<'a> {
    pub title: &'a str,
    pub database: Vec<Artist>,
    pub queue: SonikQueue,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub now_playing: Track,
    pub updating_status: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, database: Vec<Artist>) -> App<'a> {
        App {
            title,
            database,
            queue: SonikQueue::new(),
            should_quit: false,
            tabs: TabsState::new(vec!["queue", "library", "search", "browse"]),
            now_playing: Track::dummy(),
            updating_status: false
        }
    }
}
