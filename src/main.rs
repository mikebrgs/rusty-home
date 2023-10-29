extern crate chrono;

// use i2cdev::mock::MockI2CDevice;

use chrono::offset::Utc;
use chrono::DateTime;

use std::{thread, time, time::SystemTime};

use hello_i2c;

fn main() {
    // let i2c = MockI2CDevice::new();
    // let mut sensor = hello_i2c::BME280::new(i2c);

    // loop {
    //     let timestamp = SystemTime::now();
    //     let timestamp: DateTime<Utc> = timestamp.into();
    //     let timestamp = timestamp.format("%Y-%m-%dT%T");

    //     let temp = sensor.get_temperature_celsius().unwrap();
    //     let hum = sensor.get_humidity_relative().unwrap();
    //     let press = sensor.get_pressure_pascal().unwrap();

    //     println!("[{timestamp}] temperature={temp}, humididy={hum}, pressure={press}");

    //     thread::sleep(time::Duration::from_secs(3));
    // }
}
