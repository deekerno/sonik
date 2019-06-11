extern crate rusqlite;
extern crate walkdir;

use std::path::Path;
use std::fs;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use walkdir::{DirEntry, WalkDir};

use crate::database::track::Track;
use crate::database::terms::SearchQuery;

pub fn create_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists tracks 
        (
            filepath text primary key,
            title text,
            artist text,
            albumartists text,
            album text,
            year integer,
            tracknum integer,
            duration integer
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn is_music(entry: &DirEntry) -> bool {
    
    let metadata = fs::metadata(entry.path()).unwrap();
    if metadata.is_dir() {
        return false;
    }
    
    // If the filename isn't a suitable audio format, return false
    entry.file_name()
        .to_str()
        .map(|e| e.ends_with(".mp3") || e.ends_with(".flac") || e.ends_with(".ogg"))
        .unwrap_or(false)
}

pub fn update_database(conn: &Connection, music_folder: &Path) -> Result<()> {

    // Walk through the music directory and add paths for each track
    let entries = WalkDir::new(music_folder)
                    .into_iter()
                    .filter_entry(|e| is_music(e))
                    .filter_map(|v| v.ok());
   
    // Create a vector of track structs that contain ID3 information
    let tracks: Vec<Track> = entries
                            .map(|e| Track::new(e.path().to_path_buf()).unwrap())
                            .collect();

    add_tracks(conn, tracks)?;

    Ok(())
}

pub fn add_tracks(conn: &Connection, tracks: Vec<Track>) -> Result<()> {
    for track in tracks {
    
        conn.execute(
            "insert or replace into tracks
            (
                filepath,
                title,
                artist,
                albumartists,
                album,
                year,
                tracknum,
                duration
            ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[
                &track.file_path,
                &track.title,
                &track.artist,
                &track.album_artist,
                &track.album,
                &track.year.to_string(),
                &track.track_num.to_string(),
                &track.duration.to_string(),
            ]
        )?;
    }

    Ok(())
}

pub fn query_database(conn: &Connection, query: String) -> Result<Vec<Track>> {
    /*
     * In the future, the database should be
     * searchable using bangs, e.g. !y for year.
     * List of bangs:
     *  - !y - year
     *  - !yl - year less than
     *  - !yg - year greater than
     *  - !t - title
     *  - !a - artist
     *  - !ala - album artist
     *  - !al - album
     * */

    // Create a SQL query using the search terms given by the user
    let search_query = SearchQuery::new(&query);
    let mut stmt = conn.prepare(&search_query.to_sql_query())?;
    
    let results = stmt
        .query_map(NO_PARAMS, |row|
            Ok(
                Track {
                    file_path: row.get(0)?,
                    title: row.get(1)?,
                    artist: row.get(2)?,
                    album_artist: row.get(3)?,
                    album: row.get(4)?,
                    year: row.get(5)?,
                    track_num: row.get(6)?,
                    duration: row.get(7)?,
                }
            )
        )?;

    let mut tracks: Vec<Track> = Vec::new();

    for result in results {
        tracks.push(result.unwrap());
    }

    Ok(tracks)
}
