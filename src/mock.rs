extern crate chrono;

use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use hello_i2c::{veml6030, bme280};

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
        I2cTransaction::write_read(address, vec![0xF5], vec![0x00, 0x00]),  // Get stanby status
        I2cTransaction::write(address, vec![0xF5, 0x00]),  // Set standy time
        I2cTransaction::write_read(address, vec![0xF5], vec![0x00, 0x00]),  // Get filter status
        I2cTransaction::write(address, vec![0xF5, 0x00]),  // Set filter

        // Read current Mode
        // Write Mode sleep
        // Read temperature oversample
        // Write temperature oversample
        // Read current Mode
        // Write original Mode

        // Read current Mode
        // Write Mode sleep
        // Read pressure oversample
        // Write pressure oversample
        // Read current Mode
        // Write original Mode

        // Read current Mode
        // Write Mode sleep
        // Read humidity oversample
        // Write humidity oversample
        // Read current Mode
        // Write original Mode

        // Read current Mode
        // Write normal Mode

        // Read temperature
        // Read pressure
        // Read humidity
    ];
    let i2c = I2cMock::new(&expectations);

    bme280::BME280::build(i2c, address)
}


fn main() {
    let mut sensor_veml6030 = mock_veml6030();
    let mut sensor_bme280 = mock_bme280();
    
    // Ambient light sensor
    sensor_veml6030.get_ambient_light_lux().unwrap();

    // Atmospheric sensor
    sensor_bme280.get_temperature_celsius().unwrap();
    sensor_bme280.get_pressure_pascal().unwrap();
    sensor_bme280.get_humidity_relative().unwrap();

}
