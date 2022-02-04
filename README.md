# pir-player
Rust program to automatically play songs, when a GPIO-connected PIR sensor 
detects motion.

## Building from source for Raspberry Pi 0/1
- install `cross`
- run `cross build --target arm-unknown-linux-gnueabihf`

## Requirements
- PIR sensor connected via GPIO pin 4
- a folder called `music` in the same path as the program binary
- `music` folder must contain only song files, no sub directories
- `mpv` installed

## TODO
- allow for configuration of GPIO pin, folder location, music client
- enable usage of sub directories
- touchscreen-friendly frontend for pause, play, skip, etc.
- support for subsonic servers