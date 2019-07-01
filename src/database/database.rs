use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use bincode::{deserialize_from, serialize_into};
use ignore::{DirEntry, Walk};

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

pub fn create_and_load_database(
    music_folder: &Path,
    database_path: &Path,
) -> Result<Vec<Artist>, ()> {
    // create vector of artists
    let mut artists: Vec<Artist> = Vec::new();

    // Walk through the music directory and add paths for each track
    for result in Walk::new(music_folder) {
        match result {
            Ok(entry) => {
                if is_music(&entry) {
                    let track = Track::new(entry.into_path());
                    match track.ok() {
                        Some(t) => add_to_database_helper(t, &mut artists),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    let mut f =
        BufWriter::new(fs::File::create(database_path).expect("Could not write to database path"));

    // Sort for easy finding in the UI
    artists.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    serialize_into(&mut f, &artists).expect("Could not serialize database to file");

    Ok(artists)
}

pub fn load_database(database_path: &Path) -> Result<Vec<Artist>, ()> {
    let mut reader =
        BufReader::new(fs::File::open(database_path).expect("Could not open database file"));

    let artists = deserialize_from(&mut reader).expect("Could not deserialize");

    Ok(artists)
}

fn add_to_database_helper(t: Track, artists: &mut Vec<Artist>) {
    // Copy the string information out of the track and pass it
    // to add_to_database along with the actual track struct

    let artist_name = t.artist.clone();
    let album_title = t.album.clone();
    let album_year = t.year.clone();

    add_to_database(&artist_name, &album_title, album_year, t, artists);
}

fn add_to_database(
    artist_name: &String,
    album_title: &String,
    album_year: i32,
    t: Track,
    artists: &mut Vec<Artist>,
) {
    // Strings should be copies of information in track
    // Use them to add/check artists/albums and add track

    // Find an artist that matches the artist name
    let artist_index = artists
        .iter()
        .position(|a| a.name == artist_name.to_string());

    match artist_index {
        // If there is an artist that matches that name...
        Some(idx) => {
            let album_index = artists[idx]
                .albums
                .iter()
                .position(|al| al.title == album_title.to_string());
            match album_index {
                Some(al_idx) => {
                    artists[idx].albums[al_idx].update_album(t);
                }

                None => {
                    // If not, create the album and add the track
                    let mut album =
                        Album::new(album_title.to_string(), artist_name.to_string(), album_year)
                            .unwrap();
                    //debug - println!("Created new album: {}", album_title);
                    album.tracks.push(t);
                    artists[idx].add_album(album);
                }
            }
        }

        // If no artist matches that name, then create the artist and album, and add track
        None => {
            let mut artist = Artist::new(artist_name.to_string()).unwrap();
            //debug - println!("Created new artist: {}", &artist.name);

            let mut album =
                Album::new(album_title.to_string(), artist_name.to_string(), album_year).unwrap();
            //debug - println!("Created new album: {}", &album.title);
            album.tracks.push(t);
            &artist.add_album(album);
            artists.push(artist);
        }
    }
}
