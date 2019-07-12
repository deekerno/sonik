use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use bincode::{deserialize_from, serialize_into};
use hashbrown::HashMap;
use ignore::{DirEntry, Walk};
use simsearch::SimSearch;

use crate::application::config::Config;
use crate::storage::record::{Album, Artist, Record, Track};

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

pub fn create_and_load_database(config: &Config) -> Result<Vec<Artist>, ()> {
    // create vector of artists
    let mut artists: Vec<Artist> = Vec::new();

    // Walk through the music directory and add paths for each track
    for result in Walk::new(&config.music_folder) {
        if let Ok(entry) = result {
            if is_music(&entry) {
                let track = Track::new(entry.into_path());
                if let Ok(t) = track {
                    add_to_database_helper(t, &mut artists)
                }
            }
        }
    }

    let mut f = BufWriter::new(
        fs::File::create(&config.database_path).expect("Could not write to database path"),
    );

    // Sort for easy finding in the UI
    artists.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    serialize_into(&mut f, &artists).expect("Could not serialize database to file");

    Ok(artists)
}

pub fn load_database(config: &Config) -> Result<Vec<Artist>, ()> {
    let mut library_reader = BufReader::new(
        fs::File::open(&config.database_path).expect("Could not open database file"),
    );

    let artists = deserialize_from(&mut library_reader).expect("Could not deserialize");

    Ok(artists)
}

fn add_to_database_helper(t: Track, artists: &mut Vec<Artist>) {
    // Copy the string information out of the track and pass it
    // to add_to_database along with the actual track struct

    let artist_name = t.album_artist.clone();
    let album_title = t.album.clone();
    let album_year = t.year;

    add_to_database(&artist_name, &album_title, album_year, t, artists);
}

fn add_to_database(
    artist_name: &str,
    album_title: &str,
    album_year: i32,
    t: Track,
    artists: &mut Vec<Artist>,
) {
    // Strings should be copies of information in track
    // Use them to add/check artists/albums and add track

    // Find an artist that matches the artist name
    let artist_index = artists.iter().position(|a| a.name == artist_name);

    match artist_index {
        // If there is an artist that matches that name...
        Some(idx) => {
            let album_index = artists[idx]
                .albums
                .iter()
                .position(|al| al.title == album_title);
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
            artist.add_album(album);
            artists.push(artist);
        }
    }
}

pub fn create_search_map<R: Record>(
    records: &[R],
    save_path: &Path,
) -> Result<HashMap<String, usize>, ()> {
    let mut search_map = HashMap::new();

    for (i, record) in (&records).iter().enumerate() {
        let name = record.name();
        search_map.insert(name.to_lowercase(), i);
    }

    let mut map_file =
        BufWriter::new(fs::File::create(save_path).expect("Could not write to map path"));

    serialize_into(&mut map_file, &search_map).expect("Could not serialize map to file");

    Ok(search_map)
}

pub fn load_search_map(file_path: &Path) -> Result<HashMap<String, usize>, ()> {
    let mut map_reader =
        BufReader::new(fs::File::open(&file_path).expect("Could not open map file"));

    let search_map = deserialize_from(&mut map_reader).expect("Could not deserialize");

    Ok(search_map)
}

pub fn create_fuzzy_searcher<R: Record>(records: &[R]) -> Result<SimSearch<usize>, ()> {
    let mut engine: SimSearch<usize> = SimSearch::new();

    for (i, record) in (&records).iter().enumerate() {
        let name = record.name();
        engine.insert(i, name);
    }

    Ok(engine)
}

pub fn search(engine: &SimSearch<usize>, query_string: &str) -> Vec<usize> {
    let results: Vec<usize> = engine.search(query_string);

    results
}
