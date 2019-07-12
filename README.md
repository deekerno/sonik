<p align="center">
    <img alt="sonik" title="sonik" src="https://i.imgur.com/B6vYKJz.png"
    width="250">
</p>
<h1 align="center">sonik</h1><br>
<p align="center">
A music player that's gotta go fast.
</p>

## Introduction
_sonik_ is a console music player that is fast, lightweight, and elegant. It aims to play the music you want to hear as fast as you can get to it. Written in Rust, it has a small feature set in order to keep its memory footprint small. The binary size is under 4MB and usually uses ~5MB of memory. It can create an entirely new database for a large music collection in less than a second. It plays MP3, FLAC, WAV, and Vorbis file formats, and primarily depends on ID3 tags to facilitate organization.

### Note
This program is in the **alpha** stage. It is now at v0.3 as it allows for all the regular usage that you would expect from a basic music player. The search tab has been implemented; however, it only shows results for artists and does not interact with the library yet. There is some additional work to be done in trimming down the file size and possibly some speed improvements. That being said, I think it's an enjoyable experience. I use it to listen to my own collection.

## Installation
Feel free to download the latest release and run `./sonik`. Or clone the repository and run `cargo
run`. If the command is run with no flags, then the program will check your user directory for the `.sonik
` folder. If absent, the program will create the folder and write a default
configuration file (`config.toml`) that defines the music folder location at
`[home_dir]/Music`. You can specify the media location by using the `-d` flag.  It will create and write the database to the program folder as `library.db`, and will then launch the interface.

## Flags
- -d [FOLDER]: specifies the location that will be analyzed for database
    creation
- -h: print help information
- -V: version information

## Usage
| Control Keys  | Function                          |
| ------------- |----------------------------------:|
| 1-3           | switch through tabs               |
| Enter (Return)| play track now                    |
| Space         | add (track/album/artist) to queue |
| n             | play (track/album/artist) next    |
| s             | shuffle queue in place            |
| >             | next track                        |
| c             | stop track and clear the queue    |
| p             | play/pause                        |
| Esc           | quit program                      |

## TODO
- [x] create keyboard-driven interface
- [x] current queue view
- [x] library view
- [x] search view
- [x] play/pause/stop major audio formats
- [ ] seek during playback
- [x] shuffle algorithm
- [ ] repeat track/playlist/album
- [x] music database
- [x] search functionality
- [x] add multi-threading
- [ ] add logging
- [ ] add statistics

## Disclaimer
This project makes no claims about keeping your data safe from harm's way. The
program _should not_ do anything to manipulate your files in any way, but I am
no expert. Please use at your own risk.

## Special Thanks
- [rodio](https://github.com/tomaka/rodio): audio playback
- [rust-id3](https://github.com/jameshurst/rust-id3): reading of ID3 metadata
- [simsearch-rs](https://github.com/andylokandy/simsearch-rs): fuzzy search
- [tui-rs](https://github.com/fdehau/tui-rs): terminal user interface library

## License
MIT
