use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;
use serde_derive::{Serialize, Deserialize};

use id3::Tag;

use crate::database::vec_compare;

#[derive(Hash, Eq, Serialize, Deserialize, Debug)]
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

#[derive(Hash, Eq, Serialize, Deserialize, Debug)]
pub struct Album {
    pub title: String,
    pub artist: String,
    pub year: i32,
    pub tracks: Vec<Track>,
}

#[derive(Eq, Serialize, Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub albums: HashSet<Album>,
}

impl Track {
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

        Ok(
            Track {
                file_path: path.as_path().to_string_lossy().to_string(),
                title: title,
                artist: artist,
                album_artist: album_artist,
                album: album,
                year: year,
                track_num: track_num,
                duration: duration,
            }
        )
    }

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

impl Album {
    pub fn new(album_title: String, artist_name: String, release_year: i32) -> Result<Album, ()> {
        let mut tracklist: Vec<Track> = Vec::new();

        Ok(
            Album {
                title: album_title,
                artist: artist_name,
                year: release_year,
                tracks: tracklist,
            }
        )
    }

    // This will put the tracks in their correct tracklist order
    pub fn sort(mut self) {
        self.tracks.sort_by(|a, b| a.track_num.cmp(&b.track_num));
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
        self.title == other.title && vec_compare(&self.tracks, &other.tracks)
    }
}

impl Artist {
    pub fn new(artist_name: String) -> Result<Artist, ()> {
        let mut album_collection: HashSet<Album> = HashSet::new();

        Ok(
            Artist {
                name: artist_name,
                albums: album_collection,
            }
        )
    }

    pub fn add_album(mut self, album: Album) -> Result<(), ()> {
        self.albums.insert(album);

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
        self.name == other.name && (self.albums.symmetric_difference(&other.albums).count() == 0)
    }
}

