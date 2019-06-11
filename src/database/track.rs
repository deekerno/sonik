use std::path::PathBuf;

use id3::Tag;

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

impl Track {
    pub fn new(path: PathBuf) -> Result<Track, ()> {
        let tag = Tag::read_from_path(&path).unwrap();
        
        Ok(
            Track {
                file_path: path.as_path().to_string_lossy().to_string(),
                title: tag.title().unwrap().to_string(),
                artist: tag.artist().unwrap().to_string(),
                album_artist: tag.album_artist().unwrap().to_string(),
                album: tag.album().unwrap().to_string(),
                year: tag.year().unwrap(),
                track_num: tag.track().unwrap(),
                duration: tag.duration().unwrap(),
            }
        )
    }
}
