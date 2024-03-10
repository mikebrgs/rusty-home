extern crate chrono;

use i2cdev::linux::LinuxI2CDevice;

use chrono::offset::Utc;
use chrono::DateTime;

use std::{thread, time, time::SystemTime};

use hello_i2c;

fn main() {
    let i2c = LinuxI2CDevice::new("/dev/i2c-1", 0x77).unwrap();
    let mut sensor = hello_i2c::BME280::new(i2c);

    sensor.start().unwrap();
    thread::sleep(time::Duration::from_secs(1));

    loop {
        let timestamp = SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();
        let timestamp = timestamp.format("%Y-%m-%dT%T");

        let temp = sensor.get_temperature_celsius().unwrap();
        let hum = sensor.get_humidity_relative().unwrap();
        let press = sensor.get_pressure_pascal().unwrap();

        dbg!(sensor.t_fine);

        println!("[{timestamp}] temperature={temp}, humididy={hum}, pressure={press}");

        thread::sleep(time::Duration::from_secs(3));
    }
}
