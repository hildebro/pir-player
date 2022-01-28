use gpio::{GpioIn};
use std::{thread, time};

fn main() {
    // Let's open GPIO23, e.g. on a Raspberry Pi 2.
    let mut gpio4 = gpio::sysfs::SysFsGpioInput::open(4).unwrap();

    // The main thread will simply display the current value of GPIO23 every 500ms.
    loop {
        println!("GPIO 4: {:?}", gpio4.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(500));
    }
}
