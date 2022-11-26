use ::rust_gpiozero::*;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

mod distance_sensor;

fn main() -> Result<(), Box<dyn Error>> {
    println!("---Disance Sensor Tests Program---");
    let mut sensor = distance_sensor::DistanceSensor::new(18,24).unwrap();
    loop {
        let distance = sensor.get_distance();

        println!("Distance: {}cm", distance);        

        thread::sleep(Duration::from_millis(500));
    }
}
