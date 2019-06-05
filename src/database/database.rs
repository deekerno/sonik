extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use database::{Track, SearchQuery};

pub fn create_database(conn: &Connection) {
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
}

pub fn update_database(tracks: Vec<Track>) {
    // This will take ownership of the vector
    // Take each element of the vector and call add_track
}

pub fn add_track(conn: &Connection, track: &Track) {
    conn.excute(
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
            &track.file_path.to_string(),
            &track.title,
            &track.artist,
            &track.album_artist,
            &track.album,
            &track.year,
            &track.track_num,
            &track.duration,
        ]
    )?;
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

    let search_query = SearchQuery::new(query);
    let mut stmt = conn.prepare(search_query.to_sql_query())?;
    
    let tracks = stmt
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

    Ok(tracks)
}
