# pir-player
Rust program to automatically play songs, when a GPIO-connected PIR sensor 
detects motion.

## Building from source and deploying for Beaglebone Black
- install `cross`
- add a host entry for your BBB called `juker` to your ssh config
- run `bin/deploy`

## Requirements
- BBB using the recommended debian image from https://www.beagleboard.org/getting-started
- `pulseaudio` installed (for using bluetooth speakers)
- `mpg123` installed (for playback)
- `psmisc` installed (for handling processes)
- PIR sensor connected via GPIO pin 60
- placing your music into `/mnt/music`

## Scheduled process
root crontab
```
# Mount the sd card on startup
@reboot mount /dev/mmcblk0p1 /mnt
# Sadly this is required due to a bug in pulseaudio
@reboot sleep 30 && systemctl restart bluetooth
```
user crontab
```
# Every minute between 6am to 12am, connect the bluetooth speaker, if it's not connected yet.
* 6-23 * * * pacmd list-sinks | rg -q SoundCore || bluetoothctl connect 08:EB:ED:A2:E9:B9
# Every minute between 6am to 12am, start the binary, if it's not started yet.
* 6-23 * * * ps aux | rg pir-player | rg -q -v 'rg pir-player' || (cd /home/debian && /home/debian/pir-player >> /home/debian/output.log 2>&1)
# Close the player at 1am.
0 1 * * * killall pir-player && killall mpg123
```

## Debugging
Run the binary with `-d` to debug the music player.  
Run the binary with `-q` to debug the sensor.

## TODO
- allow for configuration of GPIO pin, folder location, music client
- touchscreen-friendly frontend for pause, play, skip, etc.
- support for remote file playback
