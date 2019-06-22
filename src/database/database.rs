use std::fs;

use ignore::{DirEntry, Walk};
//use rusqlite::{Connection, Result};
//use rusqlite::NO_PARAMS;

use crate::database::record::{Album, Artist, Track};
use crate::database::terms::SearchQuery;

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

pub fn create_database() -> Result<()> {

    // create vector of artists
    let mut artists: Vec<Artist> = Vec::new();

    // Walk through the music directory and add paths for each track
    for result in Walk::new(music_folder) {
        match result {
            Ok(entry) => if is_music(&entry) {
                let track = Track::new(entry.into_path());
                match track.ok() {
                    Some(t) => add_to_database_helper(t),
                    _ => (),
                }
            },
            _ => (),
        }
    }

    Ok(())
}

fn add_to_database_helper(t: Track) {

    // Copy the string information out of the track and pass it 
    // to add_to_database along with the actual track struct
    
    let artist_name = t.artist.clone();
    let album_title = t.album.clone();
    let album_year = t.year.clone();

    add_to_database(artist_name, album_title, album_year, t);
}

fn add_to_database(artist_name: String, album_title: String, album_year: i32, t: Track) {

    // Strings should be copies of information in track
    // Use them to add/check artists/albums and add track

    // Find an artist that matches the artist name
    let artist_index = artists.iter().position(|&a| a.name == artist_name);
    
    match artist_index {
        // If there is an artist that matches that name...
        Some(idx) => {
            // determine whether the album already exists for them 
            let album = artists[idx].albums.get(album_title);

            match album {
                // If it exists, add the track to the album under that artist
                Some(al) => {al.tracks.push(t);}
                // If it doesn't, create the album and then add the track
                None => {
                    artists[idx].albums.insert(Album::new(album_title, artist_name, album_year));
                    if let Some(a) = artists[idx].albums.get(album_title) {
                        a.tracks.push(t);
                    }
                }
            }
        }
        // If no artist matches that anem, then create the artist and album, and add track
        None => {
            artists.push(Artist::new(artist_name));
            artists[-1].albums.insert(Album::new(album_title, artist_name, album_year));
            artists[-1].albums.get(album_title).unwrap().tracks.push(t);
        }
    }
}

/*pub fn update_database(conn: &Connection, music_folder: &String) -> Result<()> {

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
}*/
