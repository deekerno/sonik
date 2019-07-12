use std::fs;
use std::path::{Path, PathBuf};

use dirs::home_dir;
use serde_derive::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub music_folder: String,
    pub data_folder: String,
    pub database_path: String,
    pub art_map_path: String,
}

impl Config {
    pub fn default() -> Config {
        // This bit could probably be optimized
        let mut music_location = home_dir().unwrap();
        music_location.push("Music");

        let mut data_folder = home_dir().unwrap();
        data_folder.push(".sonik");

        let mut database_path = home_dir().unwrap();
        database_path.push(".sonik");
        database_path.push("library.db");

        let mut art_map_path = home_dir().unwrap();
        art_map_path.push(".sonik");
        art_map_path.push("artists.map");

        Config {
            music_folder: music_location.to_str().unwrap().to_owned(),
            data_folder: data_folder.to_str().unwrap().to_owned(),
            database_path: database_path.to_str().unwrap().to_owned(),
            art_map_path: art_map_path.to_str().unwrap().to_owned(),
        }
    }

    pub fn new(music_location: &str) -> Result<Config, ()> {
        let mut data_folder = home_dir().unwrap();
        data_folder.push(".sonik");

        let mut database_path = home_dir().unwrap();
        database_path.push(".sonik");
        database_path.push("library.db");

        let mut art_map_path = home_dir().unwrap();
        art_map_path.push(".sonik");
        art_map_path.push("artists.map");

        let config = Config {
            music_folder: music_location.to_string(),
            data_folder: data_folder.to_str().unwrap().to_owned(),
            database_path: database_path.to_str().unwrap().to_owned(),
            art_map_path: art_map_path.to_str().unwrap().to_owned(),
        };

        let mut config_path: PathBuf = home_dir().unwrap();
        config_path.push(".sonik");
        config_path.push("config.toml");

        fs::create_dir_all(&config.data_folder).unwrap();

        // Save the configuration info to a TOML file in the data folder
        let config_as_str = toml::to_string(&config).unwrap();
        fs::write(&config_path.to_string_lossy().into_owned(), config_as_str).ok();

        Ok(config)
    }

    pub fn get_config() -> Result<Config, ()> {
        // Set path for configuration file
        let mut config_path: PathBuf = home_dir().unwrap();
        config_path.push(".sonik");
        config_path.push("config.toml");

        // Return an error if unable to write a new configuration file
        if !config_path.exists() && write_default_config(config_path.as_path()).is_none() {
            println!("Error: Could not write default config file");
            return Err(());
        }

        // Create the configuration
        let config_string = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&config_string).unwrap();

        Ok(config)
    }
}

fn write_default_config(path: &Path) -> Option<()> {
    // Get the default config and create the necessary folders
    let default_config = Config::default();
    fs::create_dir_all(default_config.data_folder).unwrap();

    // Save the configuration info to a TOML file in the data folder
    let config_as_str = toml::to_string(&Config::default()).unwrap();
    fs::write(path.to_string_lossy().into_owned(), config_as_str).ok()
}
