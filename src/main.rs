#[macro_use]
extern crate log;
extern crate simple_logger;

mod fan;
mod temp;

use std::thread;
use std::time::Duration;
use fan::Fan;

const MIN_TEMP: i32= 60;
const MAX_TEMP: i32 = 80;

fn main() {
    simple_logger::init().unwrap();

    info!("initiating fan daemon");

    let polling_frequency = 10;
    let polling_delay = Duration::from_millis(1_000 / polling_frequency);

    let fan = Fan::new().unwrap();

    loop {
        let temp = temp::read().unwrap();

        debug!("CPU Temp: {} c", temp);

        // Fan speed should smoothly transition from 0 at MIN_TEMP to 1 at MAX_TEMP
        let fan_speed = ((temp - MIN_TEMP) as f32 / (MAX_TEMP - MIN_TEMP) as f32)
            .max(0.0).min(1.0);

        fan.set_speed(fan_speed).unwrap();

        thread::sleep(polling_delay);
    }
}
