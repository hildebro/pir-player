# pir-player
Rust program to automatically play songs, when a GPIO-connected PIR sensor 
detects motion.

## Building from source for Raspberry Pi 0/1
- install `cross`
- run `cross build --target arm-unknown-linux-gnueabihf`

## Installation
- install vlc
- copy unit file to .config/systemd/user/
- enable/start the service
- for journalctl output, set `Storage` to `persistent` in `/etc/systemd/journald.conf`
