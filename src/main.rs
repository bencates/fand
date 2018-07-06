#[macro_use]
extern crate log;
extern crate simple_logger;

mod temp;

use std::thread;
use std::time::Duration;

fn main() {
    simple_logger::init().unwrap();

    info!("initiating fan daemon");

    let polling_frequency = 10;
    let polling_delay = Duration::from_millis(1_000 / polling_frequency);

    loop {
        let temp = temp::read().unwrap();

        debug!("CPU Temp: {} c", temp);

        thread::sleep(polling_delay);
    }
}
