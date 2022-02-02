use gpio::{GpioIn, GpioValue};
use std::{fs, thread, time};

fn main() {
    // Opening the occupied GPIO pin.
    let mut gpio_input = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    let paths = fs::read_dir("/srv/pir-player-rs/music").unwrap();
    let mut song_paths: Vec<String> = Vec::new();
    for path in paths {
        let song_path = path.unwrap().path().to_str().unwrap().to_string();
        song_paths.push(song_path);
    }

    loop {
        let value = gpio_input.read_value().unwrap();
        if value == GpioValue::High {
            let current_song = match song_paths.pop() {
                None => "No more songs".to_string(),
                Some(song) => song,
            };

            println!("Name: {}", current_song);
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}
