extern crate chrono;

use linux_embedded_hal::I2cdev;

use chrono::offset::Utc;
use chrono::DateTime;

use std::{thread, time, time::SystemTime};

use hello_i2c::{bme280, moisture, veml6030};

fn main() {
    let mut sensor_bme280 = bme280::BME280::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        bme280::Address::Alternative.into()
    );

    let mut sensor_moisture = moisture::Moisture::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        moisture::Address::Default.into()
    );

    let mut sensor_veml6030 = veml6030::VEML6030::build(
        I2cdev::new("/dev/i2c-1").unwrap(),
        veml6030::Address::Default.into()
    );

    thread::sleep(time::Duration::from_secs(1));

    loop {
        let timestamp = SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();
        let timestamp = timestamp.format("%Y-%m-%dT%T");

        let temp = sensor_bme280.get_temperature_celsius().unwrap();
        let hum = sensor_bme280.get_humidity_relative().unwrap();
        let press = sensor_bme280.get_pressure_pascal().unwrap();

        println!("[{timestamp}] temperature={temp}, humididy={hum}, pressure={press}");

        let moisture_level = sensor_moisture.get_moisture_level().unwrap();

        println!("[{timestamp}] moisture_level={moisture_level}");

        let lux = sensor_veml6030.get_ambient_light_lux().unwrap();

        println!("[{timestamp}] lux={lux}");

        thread::sleep(time::Duration::from_secs(3));
    }
}
