#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate sysfs_pwm;

mod temp;

use std::thread;
use std::time::Duration;
use sysfs_pwm::Pwm;

const GPIO_CHIP: u32 = 0;
const GPIO_PIN: u32 = 18;

const MIN_TEMP: i32= 60;
const MAX_TEMP: i32 = 80;

fn main() {
    simple_logger::init().unwrap();

    info!("initiating fan daemon");

    let polling_frequency = 10;
    let polling_delay = Duration::from_millis(1_000 / polling_frequency);

    let pwm = Pwm::new(GPIO_CHIP, GPIO_PIN).unwrap();
    pwm.with_exported(|| {
        loop {
            let temp = temp::read().unwrap();

            debug!("CPU Temp: {} c", temp);

            // Fan speed smoothly transitions from 0 at MIN_TEMP to 1 at MAX_TEMP
            let fan_speed = ((temp - MIN_TEMP) as f32 / (MAX_TEMP - MIN_TEMP) as f32)
                .max(0.0).min(1.0);

            let period_ns: u32 = pwm.get_period_ns().unwrap();
            let duty_cycle_ns = (fan_speed * period_ns as f32) as u32;
            pwm.set_duty_cycle_ns(duty_cycle_ns).unwrap();

            thread::sleep(polling_delay);
        }
    }).unwrap();
}
