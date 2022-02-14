use gpio::sysfs::SysFsGpioInput;
use gpio::{GpioIn, GpioValue};
use rand::{seq::SliceRandom, thread_rng};
use std::{env, fs, io, thread, time};
use std::fs::File;
use std::io::{BufReader, Write};
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--debug" {
        debug_loop(&mut gpio_input)
    } else {
        player_loop(&mut gpio_input)
    }
}

/// Just a simple loop to check, whether the PIR sensor is giving expected output.
fn debug_loop(gpio_input: &mut SysFsGpioInput) -> ! {
    loop {
        match gpio_input.read_value().unwrap() {
            GpioValue::Low => print!("."),
            GpioValue::High => print!("!")
        }
        // Must flush in order for print! to show up immediately.
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn player_loop(gpio_input: &mut SysFsGpioInput) -> ! {
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
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        // Load a sound from a file
        let file = BufReader::new(File::open(current_song).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        // The sound plays in a separate thread. This call will block the current thread until the \
        // sink has finished playing all its queued sounds.
        sink.sleep_until_end();

        // Command::new("mpv")
        //     .arg(current_song)
        //     .arg("--no-video")
        //     .stdout(Stdio::null())
        //     .status()
        //     .expect("failed to execute process");
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
