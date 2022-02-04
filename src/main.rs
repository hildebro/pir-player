use gpio::{GpioIn, GpioValue};
use rand::{seq::SliceRandom, thread_rng};
use std::{fs, process::Command, process::Stdio, thread, time};

fn main() {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    // Get all songs to play.
    let mut song_paths: Vec<String> = get_song_paths();

    println!("Waiting for motion...");

    loop {
        // Loop has a 1 second heartbeat.
        thread::sleep(time::Duration::from_millis(1000));

        if gpio_input.read_value().unwrap() != GpioValue::High {
            // Nothing to do, if no motion is detected by the sensor.
            continue;
        }

        println!("Motion detected!");

        // Get the next song to play
        let current_song = match song_paths.pop() {
            None => {
                song_paths = get_song_paths();
                song_paths.pop().unwrap()
            }
            Some(song) => song,
        };

        // Play the current song and wait until it's finished.
        println!("Now playing: {}", current_song);
        Command::new("mpv")
            .arg(current_song)
            .arg("--no-video")
            .stdout(Stdio::null())
            .status()
            .expect("failed to execute process");
        println!("Song finished, waiting for motion...");
    }
}

/// Grabs the song paths from the desired folder and puts them into a vector for convenience.
fn get_song_paths() -> Vec<String> {
    println!("Generating song paths...");
    let paths = fs::read_dir("./music").unwrap();

    let mut song_paths: Vec<String> = Vec::new();
    for path in paths {
        let song_path = path.unwrap().path().to_str().unwrap().to_string();
        song_paths.push(song_path);
    }

    // Shuffle the songs
    song_paths.shuffle(&mut thread_rng());

    song_paths
}
