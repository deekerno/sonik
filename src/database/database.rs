extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use database::Track;

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
