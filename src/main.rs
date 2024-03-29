use std::{env, fs};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use rand::{seq::SliceRandom, thread_rng};
use rusqlite::{Connection, OptionalExtension};
use sysfs_gpio::{Direction, Pin};

fn main() {
    // Create the database, if not present.
    let conn = Connection::open("songs.db").unwrap();
    conn.execute(
        "create table if not exists songs (
             position integer primary key,
             path text not null
         )",
        [],
    ).unwrap();

    // Check for command line arguments to run different functionality.
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

/// Loop for testing the music player functionality. Instead of probing the sensor, just play all
/// songs one after the other.
fn dev_loop() -> ! {
    println!("Starting loop...");
    loop {
        let song_path = get_next_song_path();
        play_song(&song_path);
        remove_song(song_path);
    }
}

/// Loop for testing the pir sensor functionality. Plays no songs, just prints either `.` or `!`
/// constantly depending on whether motion is detected at the moment.
fn quiet_loop() {
    // Initialize the GPIO pin on which the pir sensor is hooked up.
    let pir_sensor = Pin::new(60);
    pir_sensor.with_exported(|| {
        pir_sensor.set_direction(Direction::In).unwrap();
        println!("Starting loop...");
        loop {
            // Check the sensor value and print about it.
            let current_value = pir_sensor.get_value()?;
            if current_value == 1 {
                print!("!")
            } else {
                print!(".")
            }
            sleep(Duration::from_millis(100));
            // Must flush to get the output of `print!` to show up immediately.
            io::stdout().flush().unwrap();
        }
    }).unwrap()
}

/// Loop that awaits movement and then plays a song. Afterwards, return to awaiting movement.
fn player_loop() {
    // Initialize the GPIO pin on which the pir sensor is hooked up.
    let pir_sensor = Pin::new(60);
    pir_sensor.with_exported(|| {
        pir_sensor.set_direction(Direction::In).unwrap();
        println!("Starting loop...");
        loop {
            // Check the sensor value and print about it.
            let current_value = pir_sensor.get_value()?;
            if current_value == 0 {
                // Nothing to do, if no motion is detected by the sensor.
                sleep(Duration::from_secs(1));
                continue;
            }

            println!("Motion detected!");

            let song_path = get_next_song_path();
            play_song(&song_path);
            remove_song(song_path);

            println!("Song done, back to waiting for motion...");
        }
    }).unwrap()
}

fn get_next_song_path() -> String {
    let conn = Connection::open("songs.db").unwrap();
    let next_song: Option<String> = conn
        .query_row(
            "select path from songs order by position limit 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .unwrap();

    match next_song {
        Some(song) => song,
        None => populate_db(),
    }
}

/// Plays the song at the given path and waits until it's finished.
fn play_song(song_path: &String) -> () {
    println!("Now playing: {}", song_path);
    Command::new("mpg123")
        // TODO call `id -u` to get the right value for this and save it via lazy static
        .env("XDG_RUNTIME_DIR", "/run/user/1000")
        .arg("--no-control")
        .arg("-q")
        .arg(&song_path)
        .status()
        .expect("failed to execute process");
}

/// Remove the given song from the database.
/// TODO Use position instead as it is the primary key.
fn remove_song(song_path: String) -> () {
    let conn = Connection::open("songs.db").unwrap();
    conn.execute("delete from songs where path = ?", [song_path])
        .unwrap();
}

/// Fill the database with song paths in random order and return the first one.
fn populate_db() -> String {
    println!("Populate database with songs from directory...");
    // TODO Create config file for setting the music directory
    let paths = fs::read_dir("/mnt/music").unwrap();

    // Collect the song paths in a vector.
    let mut song_paths: Vec<String> = Vec::new();
    for path in paths {
        let path_buf = path.unwrap().path();
        resolve_music_files(&mut song_paths, path_buf);
    }

    if song_paths.len() == 0 {
        eprintln!("Music directory is empty!");
        std::process::exit(exitcode::DATAERR);
    }

    // Shuffle the songs.
    song_paths.shuffle(&mut thread_rng());

    // Add all shuffled songs to the database.
    let conn = Connection::open("songs.db").unwrap();
    for song_path in song_paths.clone() {
        conn.execute("insert into songs (path) values (?)", [song_path])
            .unwrap();
    }

    song_paths.first().unwrap().to_string()
}

/// Check the given path (including sub directories) for mp3 files and add them into song_paths.
fn resolve_music_files(song_paths: &mut Vec<String>, path_buf: PathBuf) {
    if path_buf.is_dir() {
        let sub_paths = path_buf.read_dir().unwrap();
        for sub_path in sub_paths {
            resolve_music_files(song_paths, sub_path.unwrap().path())
        }
    }

    // Only put mp3 files into the vector (so files like album cover images are ignored).
    let file_path = path_buf.to_str().unwrap().to_string();
    if file_path.ends_with(".mp3") {
        song_paths.push(file_path);
    }
}
