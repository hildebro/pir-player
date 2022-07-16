use std::io::Write;
use std::{env, fs, io, process::Command, process::Stdio, thread, time};

use gpio::{GpioIn, GpioValue};
use rand::{seq::SliceRandom, thread_rng};
use rusqlite::{Connection, OptionalExtension};

fn main() -> ! {
    // Create the database, if not present.
    let conn = Connection::open("songs.db").unwrap();
    conn.execute(
        "create table if not exists songs (
             position integer primary key,
             path text not null
         )",
        [],
    )
    .unwrap();

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
fn quiet_loop() -> ! {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    println!("Starting loop...");
    loop {
        match gpio_input.read_value().unwrap() {
            GpioValue::Low => print!("."),
            GpioValue::High => print!("!"),
        }
        // Must flush to get the output of `print!` to show up immediately.
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}

/// Loop that awaits movement and then plays a song. Afterwards, return to awaiting movement.
fn player_loop() -> ! {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    println!("Starting loop...");
    loop {
        if gpio_input.read_value().unwrap() != GpioValue::High {
            // Nothing to do, if no motion is detected by the sensor.
            thread::sleep(time::Duration::from_millis(1000));
            continue;
        }

        println!("Motion detected!");

        let song_path = get_next_song_path();
        play_song(&song_path);
        remove_song(song_path);

        println!("Waiting for motion...");
    }
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
    Command::new("cvlc")
        .arg(&song_path)
        .arg("vlc://quit")
        .stdout(Stdio::null())
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
    let paths = fs::read_dir("./music").unwrap();

    // Collect the song paths in a vector.
    let mut song_paths: Vec<String> = Vec::new();
    for path in paths {
        let song_path = path.unwrap().path().to_str().unwrap().to_string();
        song_paths.push(song_path);
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
