# pir-player
Rust program to automatically play songs, when a GPIO-connected PIR sensor 
detects motion.

## Building from source for Raspberry Pi 0/1
- install `cross`
- run `cross build --target arm-unknown-linux-gnueabihf`

## Requirements
- PIR sensor connected via GPIO pin 4
- a folder called `music` in the working directory
- `music` folder must contain only song files, no sub directories
- `mpv` installed

## Debugging
Running the binary with `--debug` will print PIR sensor readings.

## TODO
- allow for configuration of GPIO pin, folder location, music client
- enable usage of sub directories
- touchscreen-friendly frontend for pause, play, skip, etc.
- support for subsonic servers
