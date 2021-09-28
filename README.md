# pir-player
Script for raspberry pi to play songs when a motion detector triggers

## Installation
- install vlc
- copy unit file to .config/systemd/user/
- enable/start the service
- for journalctl output, set `Storage` to `persistent` in `/etc/systemd/journald.conf`
