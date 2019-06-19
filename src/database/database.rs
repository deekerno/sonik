use std::fs;

use ignore::{DirEntry, Walk};
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use crate::database::record::{Album, Artist, Track};
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

    // Add all tracks to database
    
    // Select distinct artists from tracks table
    
    // Select 

    Ok(())
}

fn is_music(entry: &DirEntry) -> bool {
    
    let metadata = fs::metadata(entry.path()).unwrap();
    if metadata.is_dir() {
        return false;
    }

    // If the filename isn't a suitable audio format, return false
    if let Some(extension) = entry.path().extension() {
        match extension.to_str() {
            Some("mp3") => return true,
            Some("flac") => return true,
            Some("ogg") => return true,
            _ => return false,
        };
    } else {
        return false;
    }
}

pub fn update_database(conn: &Connection, music_folder: &String) -> Result<()> {

    let mut tracks: Vec<Track> = Vec::new();

    // Walk through the music directory and add paths for each track
    for result in Walk::new(music_folder) {
        match result {
            Ok(entry) => if is_music(&entry) {
                let track = Track::new(entry.into_path());
                match track.ok() {
                    Some(t) => tracks.push(t),
                    _ => (),
                }
            },
            _ => (),
        }
    }

    add_tracks(conn, tracks)?;

    Ok(())
}

pub fn add_tracks(conn: &Connection, tracks: Vec<Track>) -> Result<()> {
    
    // Add each track's info to the database
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
