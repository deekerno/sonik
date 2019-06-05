use std::path::Path;
use std::str::FromStr;

use id3::Tag;

pub struct Track {
    pub file_path: Path,
    pub title: String,
    pub artist: String,
    pub album_artist: String,
    pub album: String,
    pub year: i32,
    pub track_num: i32,
    pub duration: i32,
}

impl Track {
    pub fn new(path: Path) -> Result<Track> {
        let tag = Tag::read_from_path(&path)?;
        let file_path = &path;
        let title = tag.title().unwrap();
        let artist = tag.artist().unwrap();
        let album_artist = tag.album_artist().unwrap();
        let album = tag.album().unwrap();
        let year = tag.year().unwrap();
        let track_num = tag.track().unwrap();
        let duration = tag.duration().unwrap();
    }
}
