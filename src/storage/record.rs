use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use id3::Tag;
use serde_derive::{Deserialize, Serialize};

use crate::storage::vec_compare;

#[derive(Clone, Eq, Serialize, Deserialize, Debug)]
pub struct Track {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub album_artist: String,
    pub album: String,
    pub year: i32,
    pub track_num: u32,
    pub duration: u32,
}

#[derive(Clone, Eq, Serialize, Deserialize, Debug)]
pub struct Album {
    pub title: String,
    pub artist: String,
    pub year: i32,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Eq, Serialize, Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub albums: Vec<Album>,
}

pub trait Record {
    fn name(&self) -> &str;
}

impl Track {
    // Should probably implement a Default for this

    pub fn new(path: PathBuf) -> Result<Track, ()> {
        // Some paths aren't UTF-8 compliant
        // For now, we will ignore these tracks
        let tag = Tag::read_from_path(&path);
        match tag.ok() {
            None => return Err(()),
            _ => (),
        }

        // Anything that gets to this stage has a safe path
        let safe_tag = Tag::read_from_path(&path).unwrap();

        let mut title: String = "".to_string();
        if let Some(s) = safe_tag.title() {
            title = s.to_string();
        }

        let mut artist: String = "".to_string();
        if let Some(s) = safe_tag.artist() {
            artist = s.to_string();
        }

        let mut album: String = "".to_string();
        if let Some(s) = safe_tag.album() {
            album = s.to_string();
        }

        let mut album_artist: String = "".to_string();
        if let Some(s) = safe_tag.album_artist() {
            album_artist = s.to_string();
        }

        let mut year: i32 = 0;
        if let Some(x) = safe_tag.year() {
            year = x;
        }

        let mut track_num: u32 = 0;
        if let Some(x) = safe_tag.track() {
            track_num = x;
        }

        let mut duration: u32 = 0;
        if let Some(x) = safe_tag.duration() {
            duration = x;
        }

        Ok(Track {
            file_path: path.as_path().to_string_lossy().to_string(),
            title, 
            artist,
            album_artist,
            album,
            year,
            track_num,
            duration,
        })
    }

    // This is implemented mainly to have a blank now playing on startup
    pub fn dummy() -> Track {
        Track {
            file_path: "".to_string(),
            title: "".to_string(),
            artist: "".to_string(),
            album_artist: "".to_string(),
            album: "".to_string(),
            year: 0,
            track_num: 0,
            duration: 0,
        }
    }
}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Track) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Track) -> Ordering {
        self.track_num.cmp(&other.track_num)
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.file_path == other.file_path
    }
}

impl Record for Track {
    fn name(&self) -> &str {
        self.title.as_str()
    }
}

impl Album {
    pub fn new(album_title: String, artist_name: String, release_year: i32) -> Result<Album, ()> {
        let tracklist: Vec<Track> = Vec::new();

        Ok(Album {
            title: album_title,
            artist: artist_name,
            year: release_year,
            tracks: tracklist,
        })
    }

    pub fn update_album(&mut self, t: Track) -> Result<(), ()> {
        self.tracks.push(t);

        self.tracks.sort_by(|a, b| a.track_num.cmp(&b.track_num));

        Ok(())
    }
}

impl PartialOrd for Album {
    fn partial_cmp(&self, other: &Album) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Album {
    fn cmp(&self, other: &Album) -> Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        // Tracklists are compared to make sure albums are different
        // even if they share the same name, e.g. regular vs. deluxe albums
        self.title == other.title && vec_compare(&self.tracks, &other.tracks)
    }
}

impl Hash for Album {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

// Implementing borrow for album title so that it can be used
// as a query type in the hash set inside an artist
impl Borrow<String> for Album {
    fn borrow(&self) -> &String {
        &self.title
    }
}

impl Record for Album {
    fn name(&self) -> &str {
        self.title.as_str()
    }
}

impl Artist {
    pub fn new(artist_name: String) -> Result<Artist, ()> {
        let album_collection: Vec<Album> = Vec::new();

        Ok(Artist {
            name: artist_name,
            albums: album_collection,
        })
    }

    pub fn add_album(&mut self, album: Album) -> Result<(), ()> {
        self.albums.push(album);

        self.albums
            .sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

        Ok(())
    }
}

impl PartialOrd for Artist {
    fn partial_cmp(&self, other: &Artist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Artist {
    fn cmp(&self, other: &Artist) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.name == other.name && (vec_compare(&self.albums, &other.albums))
    }
}

impl Record for Artist {
    fn name(&self) -> &str {
        &self.name[..]
    }
}
