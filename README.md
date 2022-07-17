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
- `vlc` installed

## Scheduled process
If you don't want the player to be active at night, use this crontab:
```
0 6 * * * cd /home/pi && /home/pi/pir-player >> /home/pi/output.log 2>&1
0 1 * * * killall /home/pi/pir-player && killall cvlc
```

## Debugging
Run the binary with `-d` to debug the music player.  
Run the binary with `-q` to debug the sensor.

## TODO
- allow for configuration of GPIO pin, folder location, music client
- enable usage of sub directories
- touchscreen-friendly frontend for pause, play, skip, etc.
- support for remote file playback
