use ::rust_gpiozero::*;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

mod distance_sensor;

const THRESHOLD: f64 = 20.0;

fn main() -> Result<(), Box<dyn Error>> {
    let led = LED::new(17);
    println!("---Disance Sensor Tests Program---");
    let mut sensor = distance_sensor::DistanceSensor::new(18,24).unwrap();
    loop {
        let distance = sensor.get_distance();

        println!("Distance: {}cm", distance);        
        if distance >= 0.0 && distance <= THRESHOLD  {
            led.on();
        } else {
            led.off();
        }

        thread::sleep(Duration::from_millis(500));
    }
}
