use std::io::Write;
use std::{env, fs, io, process::Command, process::Stdio, thread, time};

use gpio::{GpioIn, GpioValue};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        player_loop()
    }

    let option = args[1].as_str();
    match option {
        "-q" => quiet_loop(),
        "-d" => dev_loop(),
        _ => {
            eprintln!("Unexpected option: {}", option);
            std::process::exit(exitcode::IOERR);
        }
    }
}

/// Loop for testing the music player functionality. Instead of probing the sensor, just play songs
/// one after the other.
fn dev_loop() -> ! {
    let mut song_paths: Vec<String> = get_song_paths();

    loop {
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
        Command::new("cvlc")
            .arg(current_song)
            .arg("vlc://quit")
            .stdout(Stdio::null())
            .status()
            .expect("failed to execute process");
        println!("Song finished, waiting for motion...");
    }
}

/// Just a simple loop to check, whether the PIR sensor is giving expected output. Instead of
/// playing a song on movement, the program prints `!` in the terminal.
fn quiet_loop() -> ! {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();
    loop {
        match gpio_input.read_value().unwrap() {
            GpioValue::Low => print!("."),
            GpioValue::High => print!("!"),
        }
        // Must flush in order for print! to show up immediately.
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}

/// Loop that awaits movement and then plays a song. Afterwards, return to awaiting movement.
fn player_loop() -> ! {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    let mut song_paths: Vec<String> = get_song_paths();

    println!("Starting loop...");
    loop {
        if gpio_input.read_value().unwrap() != GpioValue::High {
            // Nothing to do, if no motion is detected by the sensor.
            thread::sleep(time::Duration::from_millis(1000));
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
        Command::new("cvlc")
            .arg(current_song)
            .arg("vlc://quit")
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

    if song_paths.len() == 0 {
        eprintln!("Music directory is empty!");
        std::process::exit(exitcode::DATAERR);
    }

    // Shuffle the songs
    song_paths.shuffle(&mut thread_rng());

    song_paths
}
