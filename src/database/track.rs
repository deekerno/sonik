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
