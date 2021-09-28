from gpiozero import MotionSensor
import time
import vlc
import os, random

# Declare music directory
music_dir = "/home/pi/music"


def main():
    # Set seed
    random.seed(int(time.time()))

    # Declare motion sensor
    pir = MotionSensor(4)

    # Declare music player
    vlc_instance = vlc.Instance()
    player = vlc_instance.media_player_new()

    while True:
        print("Checking for motion...")
        pir.wait_for_motion()
        print("Movement detected!")
        song_path = random_lofi()
        player.set_mrl(song_path)
        print("Now playing " + song_path)
        player.play()
        # The player needs a moment to actually start,
        # so we need to wait for `is_playing` to actually be true.
        time.sleep(2)
        while player.is_playing():
            # Wait while the song is playing
            time.sleep(2)

        print("Song is over.")


def random_song():
    album_folders = os.listdir(music_dir)
    album = random.choice(album_folders)
    song_files = os.listdir(music_dir + "/" + album)
    song = random.choice(song_files)

    return music_dir + "/" + album + "/" + song


def random_lofi():
    lofi_folder = music_dir + "/lofi"
    lofi_songs = os.listdir(lofi_folder)
    song = random.choice(lofi_songs)

    return lofi_folder + "/" + song


def test():
    return "test.mp3"


if __name__ == '__main__':
    main()
