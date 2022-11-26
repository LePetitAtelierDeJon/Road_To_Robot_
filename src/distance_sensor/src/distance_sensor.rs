//! Structure to use a HC-SR04 ultrasonic sensor as a distance sensor in Rust on a Rapsberry Pi.

use rppal::gpio::{Gpio, OutputPin, InputPin, Error};
use std::thread;
use std::time::{Duration, Instant};

const SPEED_OF_SOUND:f64 = 0.034029; //cm/micro second 

/// Structure for the distance sensor.
pub struct DistanceSensor {
    trigger: OutputPin,
    echo: InputPin
}

impl DistanceSensor {
    /// Construct a new DistanceSensor.
    /// 
    /// [`trigger_pin`]: pin used to trigger the measure.
    /// [`echo_pin`]: pin used to receive the echo.
    pub fn new(trigger_pin: u8, echo_pin: u8) -> Result<DistanceSensor, Error> {
        // Create the gpio handle.
        let gpio = Gpio::new()?;

        // Configure the trigger pin.
        let trigger = gpio.get(trigger_pin)?.into_output();

        // Configure thre echo pin.
        let echo = gpio.get(echo_pin)?.into_input();

        // Return the distance sensor.
        Ok(DistanceSensor { echo, trigger })
    }

    /// Make a measurement with the sensor to return the distance of the closest object. If no object is found it returns -1.0.
    pub fn get_distance(&mut self) ->f64 {
        // Start the measure.
        self.trigger.set_low();
        thread::sleep(Duration::from_micros(2));
        self.trigger.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trigger.set_low();

        let mut init = Instant::now();
        let mut start = Instant::now();
        let mut duration = Duration::new(0, 0);

        // Wait the return of the ultrasound.
        while self.echo.is_low() {
            if init.elapsed().as_millis() > 30 {
                return -1.0;
            }
        }

        start = Instant::now();
        init = Instant::now();

        // Listen to echo to get the duration corresponding to the distance.
        while self.echo.is_high() {
            duration = start.elapsed();
            if init.elapsed().as_millis() > 30 {
                return -1.0;
            }
        }
        let micros = duration.as_micros(); // Convert duration to microseconds.

        // Convert the duration to a distance.
        (SPEED_OF_SOUND * micros as f64) / 2.0
    }
}