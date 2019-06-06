use std::default::Default;
use std::fs;
use std::path::{Path, PathBuf};

use app_dirs2::*;
use dirs::home_dir;
use serde_derive::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub music_folder: String,
}

impl Default for Config {
    fn default() -> Config {
        let mut home_directory = home_dir().unwrap();
        home_directory.push("Music");
        Config {
            music_folder: home_directory.to_str().unwrap().to_owned(),
        }
    }
}

fn write_default_config(path: &Path) -> Option<()> {
    let default_config = toml::to_string(&Config::default()).unwrap();
    fs::write(path.to_string_lossy().into_owned(), default_config).ok()
}

pub fn get_config() -> Result<Config, ()> {
    let mut config_path: PathBuf = home_dir().unwrap();
    config_path.push(".sonik");
    config_path.push("config.toml");
    if !config_path.exists() && write_default_config(config_path.as_path()).is_none() {
        println!("Error: Could not write default config file");
        return Err(());
    }

    let config_string = fs::read_to_string(&config_path).unwrap();
    let config = toml::from_str(&config_string);
    
    if let Ok(config) = config {
        Ok(config)
    } else {
        Err(())
    }
}
