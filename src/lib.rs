// Strong inspiration from the sparkfun python library for the Qwiic BME280 sensor on:
// https://github.com/sparkfun/Qwiic_BME280_Py
// and from the Bosch BME280 manual.

mod sensors;

pub use sensors::bme280::BME280;