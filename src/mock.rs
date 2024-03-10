extern crate chrono;

use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use hello_i2c::{bme280::{self, constants::values}, moisture, veml6030};

fn mock_veml6030() -> veml6030::VEML6030<I2cMock> {
    let address: u8 = veml6030::Address::Default.into();
    let expectations = [
        I2cTransaction::write_read(address, vec![0x00], vec![0x00, 0x00]),
        I2cTransaction::write(address, vec![0x00, 0x00, 0x00]),
        I2cTransaction::write_read(address, vec![0x00], vec![0x00, 0x00]),
        I2cTransaction::write(address, vec![0x00, 0x18, 0x00]),
        I2cTransaction::write_read(address, vec![0x00], vec![0x18, 0x00]),
        I2cTransaction::write(address, vec![0x00, 0x1A, 0x00]),
        I2cTransaction::write_read(address, vec![0x04], vec![0x00, 0xFF]),
        I2cTransaction::write_read(address, vec![0x00], vec![0x18, 0x00]),
        I2cTransaction::write_read(address, vec![0x00], vec![0x1A, 0x00]),
    ];
    let i2c = I2cMock::new(&expectations);

    veml6030::VEML6030::build(i2c, address)
}

fn mock_bme280() -> bme280::BME280<I2cMock> {
    let address: u8 = bme280::Address::Default.into();
    let expectations = [
        // Temperature calibration
        // T1 calibration
        I2cTransaction::write_read(address, vec![0x88], ((28485_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x89], ((28485_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // T2 calibration
        I2cTransaction::write_read(address, vec![0x8A], ((26735_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x8B], ((26735_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // T3 calibration
        I2cTransaction::write_read(address, vec![0x8C], ((50_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x8D], ((50_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2

        // Pressure calibration
        // P1 calibration
        I2cTransaction::write_read(address, vec![0x8E], ((36738_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x8F], ((36738_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P2 calibration
        I2cTransaction::write_read(address, vec![0x90], ((-10635_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x91], ((-10635_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P3 calibration
        I2cTransaction::write_read(address, vec![0x92], ((3024_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x93], ((3024_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P4 calibration
        I2cTransaction::write_read(address, vec![0x94], ((6980_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x95], ((6980_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P5 calibration
        I2cTransaction::write_read(address, vec![0x96], ((-4_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x97], ((-4_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P6 calibration
        I2cTransaction::write_read(address, vec![0x98], ((-7_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x99], ((-7_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P7 calibration
        I2cTransaction::write_read(address, vec![0x9A], ((9900_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x9B], ((9900_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P8 calibration
        I2cTransaction::write_read(address, vec![0x9C], ((-10230_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x9D], ((-10230_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // P9 calibration
        I2cTransaction::write_read(address, vec![0x9E], ((4285_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0x9F], ((4285_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2

        // Humidity calibration
        // H1 calibration
        I2cTransaction::write_read(address, vec![0xA1], ((75_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        // H2 calibration
        I2cTransaction::write_read(address, vec![0xE1], ((365_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0xE2], ((365_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // H3 calibration
        I2cTransaction::write_read(address, vec![0xE3], ((0_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        // H4 calibration
        I2cTransaction::write_read(address, vec![0xE4], ((312_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0xE5], ((312_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // H5 calibration
        I2cTransaction::write_read(address, vec![0xE6], ((50_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1
        I2cTransaction::write_read(address, vec![0xE5], ((50_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),  // T2 part2
        // H6 calibration
        I2cTransaction::write_read(address, vec![0xE7], ((30_i64 & 0xFF) as u8).to_be_bytes().to_vec()),  // T1 part1

        // Standby
        I2cTransaction::write_read(address, vec![0xF5], vec![0x00]),  // Get stanby status
        I2cTransaction::write(address, vec![0xF5, 0x00]),  // Set standy time
        // Filter
        I2cTransaction::write_read(address, vec![0xF5], vec![0x00]),  // Get filter status
        I2cTransaction::write(address, vec![0xF5, 0x00]),  // Set filter

        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read temperature oversample
        I2cTransaction::write(address, vec![0xF4, 0x01 << 5]),  // Write temperature oversample 0x1
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep

        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read pressure oversample
        I2cTransaction::write(address, vec![0xF4, 0x01 << 2]),  // Write pressure oversample 0x1
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep

        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep
        I2cTransaction::write_read(address, vec![0xF2], vec![0x00]),  // Read humidity oversample
        I2cTransaction::write(address, vec![0xF2, 0x01]),  // Write humidity oversample 0x1
        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x00]),  // Write Mode sleep

        I2cTransaction::write_read(address, vec![0xF4], vec![0x00]),  // Read current Mode
        I2cTransaction::write(address, vec![0xF4, 0x03]),  // Write Mode normal

        // Read temperature
        I2cTransaction::write_read(address, vec![0xFA], vec![0x00]),
        I2cTransaction::write_read(address, vec![0xFB], vec![0x00]),
        I2cTransaction::write_read(address, vec![0xFC], vec![0x00]),
        // Read pressure
        I2cTransaction::write_read(address, vec![0xF7], vec![0x00]),
        I2cTransaction::write_read(address, vec![0xF8], vec![0x00]),
        I2cTransaction::write_read(address, vec![0xF9], vec![0x00]),
        // Read humidity
        I2cTransaction::write_read(address, vec![0xFD], vec![0x0F]),
        I2cTransaction::write_read(address, vec![0xFE], vec![0x00]),
    ];
    let i2c = I2cMock::new(&expectations);

    bme280::BME280::build(i2c, address)
}

fn mock_moisture() -> moisture::Moisture<I2cMock> {
    let address: u8 = moisture::Address::Default.into();
    let expectations = [
        I2cTransaction::write_read(address, vec![0x05], vec![0x00, 0x00]),
    ];
    let i2c = I2cMock::new(&expectations);

    moisture::Moisture::build(i2c, address)
}



fn main() {
    let mut sensor_veml6030 = mock_veml6030();
    let mut sensor_bme280 = mock_bme280();
    let mut sensor_moisture = mock_moisture();
    
    // Ambient light sensor
    let value = sensor_veml6030.get_ambient_light_lux().unwrap();
    println!("Lux value: {}", value);

    // Moisture sensor
    let value = sensor_moisture.get_moisture_level().unwrap();
    println!("Moisture level: {}", value);

    // Atmospheric sensor
    let value = sensor_bme280.get_temperature_celsius().unwrap();
    println!("Temperature value: {}", value);
    let value = sensor_bme280.get_pressure_pascal().unwrap();
    println!("Pressure value: {}", value);
    let value = sensor_bme280.get_humidity_relative().unwrap();
    println!("Humidity value: {}", value);
}
