<p align="center">
    <img alt="sonik" title="sonik" src="https://i.imgur.com/B6vYKJz.png"
    width="250">
</p>
<h1 align="center">sonik</h1><br>
<p align="center">
A music player that's gotta go fast.
</p>

## Introduction
_sonik_ is a console music player that is fast, lightweight, and elegant. It aims to play the music you want to hear as fast as you can get to it. Written in Rust, it has a small feature set in order to keep its memory footprint small.

### Note
This program is in the alpha stage. It is now at v0.1 as it allows for the
minimally viable usage of exploring the library and immediate playback. It
currently does not support playing from the queue nor are the search or browse
tabs implemented.

## Installation
Coming soon!

## Usage
| Control Keys  | Function           |
| ------------- |-------------------:|
| 1-4           | switch through tabs|
| Enter (Return)| play song now      |
| Space         | add to queue       |
| s             | shuffle queue      |
| < or >        | previous or next   |
| c             | clear the queue    |
| p             | play/pause         |

## TODO
- [x] create keyboard-driven interface
- [x] current queue view
- [x] library view
- [ ] search view
- [ ] file browser view
- [x] play/pause/stop major audio formats
- [ ] seek during playback
- [x] shuffle algorithm
- [ ] repeat track/playlist/album
- [x] music database
- [ ] search functionality
- [ ] add multi-threading

## Disclaimer
This project makes no claims about keeping your data safe from harm's way. The
program _should not_ do anything to manipulate your files in any way, but I am
no expert. Please use at your own risk.

## License
MIT
