use std::u8;

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod calibration;
pub mod i2c;
pub mod constants;

use constants::{values, addresses};

use crate::protocols::i2c::I2CWrapper;

pub enum Mode {
    Sleep,
    Forced,
    Normal
}

impl From<u8> for Mode {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Sleep,
            1 => Self::Forced,
            2 => Self::Forced,
            3 => Self::Normal,
            _ => panic!("Not expected")
        }
    }
}

impl From<Mode> for u8 {
    fn from(item: Mode) -> u8 {
        match item {
            Mode::Sleep => 0,
            Mode::Forced => 1,
            Mode::Normal => 3
        }
    }
}

pub enum Oversampling {
    Skipped,
    Ox1,
    Ox2,
    Ox4,
    Ox8,
    Ox16
}

impl From<u8> for Oversampling {
    fn from(value: u8) -> Self {
        match value {
            0 => Oversampling::Skipped,
            1 => Oversampling::Ox1,
            2 => Oversampling::Ox2,
            3 => Oversampling::Ox4,
            4 => Oversampling::Ox8,
            _ => Oversampling::Ox16
        }
    }
}

impl From<Oversampling> for u8 {
    fn from(value: Oversampling) -> Self {
        match value {
            Oversampling::Skipped => 0,
            Oversampling::Ox1 => 1,
            Oversampling::Ox2 => 2,
            Oversampling::Ox4 => 3,
            Oversampling::Ox8 => 4,
            Oversampling::Ox16 => 5,
        }
    }
}

pub enum StandyTime {
    Ms0_5,
    Ms62_5,
    Ms125,
    Ms250,
    Ms500,
    Ms1000,
    Ms10,
    Ms20
}

impl From<u8> for StandyTime {
    fn from(value: u8) -> Self {
        match value {
            0 => StandyTime::Ms0_5,
            1 => StandyTime::Ms62_5,
            2 => StandyTime::Ms125,
            3 => StandyTime::Ms250,
            4 => StandyTime::Ms500,
            5 => StandyTime::Ms1000,
            6 => StandyTime::Ms10,
            7 => StandyTime::Ms20,
            _ => panic!("Invalid standby value")
        }
    }
}

impl From<StandyTime> for u8 {
    fn from(value: StandyTime) -> Self {
        match value {
             StandyTime::Ms0_5 => 0,
             StandyTime::Ms62_5 => 1,
             StandyTime::Ms125 => 2,
             StandyTime::Ms250 => 3,
             StandyTime::Ms500 => 4,
             StandyTime::Ms1000 => 5,
             StandyTime::Ms10 => 6,
             StandyTime::Ms20 => 7
        }
    }
}

pub enum Filter {
    Off,
    C2,
    C4,
    C8,
    C16
}

impl From<u8> for Filter {
    // Expects 3 bits only
    fn from(value: u8) -> Self {
        match value & 0x7 {
            0 => Filter::Off,
            1 => Filter::C2,
            2 => Filter::C4,
            3 => Filter::C8,
            _ => Filter::C16,
        }
    }
}

impl From<Filter> for u8 {
    fn from(value: Filter) -> Self {
        match value {
            Filter::Off => 0,
            Filter::C2 => 1,
            Filter::C4 => 2,
            Filter::C8 => 3,
            Filter::C16 => 4,
        }
    }
}

pub enum Address {
    Default,
    Alternative
}

impl From<Address> for u8 {
    fn from(value: Address) -> Self {
        match value {
            Address::Default => addresses::DEFAULT,
            Address::Alternative => addresses::ALTERNATIVE
        }
    }
}


pub struct BME280<I2C> {
    dev: I2CWrapper<I2C>,
    calibration: calibration::Calibration,
    t_fine: i32,
}

impl<I2C: Write + WriteRead> BME280<I2C> {

    // Create new BME280 device wrapper for I2C communication.
    pub fn new(dev: I2C, address: u8) -> BME280<I2C> {
        let mut wrapper = I2CWrapper::new(dev, address);
        let calibration = calibration::Calibration::build(&mut wrapper);
        BME280 { dev: wrapper, calibration: calibration, t_fine: 0 }
    }

    pub fn build(dev: I2C, address: u8) -> BME280<I2C> {
        let mut sensor = BME280::new(dev, address);
        sensor.start().unwrap();
        sensor
    }

    // Start all parameters from for the sensor
    pub fn start(&mut self) -> Result<(), String> {
        self.set_standby_time(StandyTime::Ms0_5).unwrap();
        self.set_filter(Filter::Off).unwrap();
        self.set_temperature_oversample(Oversampling::Ox1).unwrap();
        self.set_pressure_oversample(Oversampling::Ox1).unwrap();
        self.set_humidity_oversample(Oversampling::Ox1).unwrap();
        self.set_mode(Mode::Normal).unwrap();
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.set_mode(Mode::Sleep).unwrap();
        Ok(())
    }

    // Get the ID of the chip
    pub fn get_id(&mut self) -> Result<u8, String> {
        let id = i2c::read_id(&mut self.dev);
        if id != values::CHIP_ID {
            return Err(String::from("ID doesn't match specs."))
        }
        Ok(id)
    }

    // Reset device
    pub fn reset(&mut self) -> Result<(), String> {
        i2c::write_reset(&mut self.dev);
        Ok(())
    }

    // Get mode from the device
    pub fn get_mode(&mut self) -> Result<Mode, String> {
        let mode = i2c::read_mode(&mut self.dev);
        Ok(Mode::from(mode))
    }

    // Set the mode on the device
    pub fn set_mode(&mut self, mode: Mode) -> Result<(),String> {
        i2c::write_mode(&mut self.dev, u8::from(mode));
        Ok(())
    }

    // Is the device measuring
   pub fn is_measuring(&mut self) -> Result<bool, String> {
        Ok(i2c::read_measuring_bit(&mut self.dev) == 1)
    }

    // Is the device copying NVM data to image registers
    pub fn is_updating(&mut self) -> Result<bool, String> {
        // Check bit 0 is set to 1
        Ok((i2c::read_updating_bit(&mut self.dev)) == 1)
    }

    pub fn set_humidity_oversample(&mut self, rate: Oversampling) -> Result<(), String> {
        let device_mode = self.get_mode().unwrap();
        self.set_mode(Mode::Sleep).unwrap();
        i2c::write_humidity_oversample(&mut self.dev, u8::from(rate));
        self.set_mode(device_mode).unwrap();
        Ok(())
    }

    pub fn set_temperature_oversample(&mut self, rate: Oversampling) -> Result<(), String> {
        let device_mode = self.get_mode().unwrap();
        self.set_mode(Mode::Sleep).unwrap();
        i2c::write_temperature_oversample(&mut self.dev, u8::from(rate));
        self.set_mode(device_mode).unwrap();
        Ok(())
    }

    pub fn set_pressure_oversample(&mut self, rate: Oversampling) -> Result<(), String> {
        let device_mode = self.get_mode().unwrap();
        self.set_mode(Mode::Sleep).unwrap();
        i2c::write_pressure_oversample(&mut self.dev, u8::from(rate));
        self.set_mode(device_mode).unwrap();

        Ok(())
    }

    pub fn set_standby_time(&mut self, standby: StandyTime) -> Result<(), String> {
        i2c::write_standby_time(&mut self.dev, u8::from(standby));
        Ok(())
    }

    pub fn set_filter(&mut self, filter: Filter) -> Result<(), String> {
        i2c::write_filter(&mut self.dev, u8::from(filter));
        Ok(())
    }

    // Get temperature from the sensor.
    pub fn get_temperature_celsius(&mut self) -> Result<f64, String> {
        let adc_t = i2c::get_temperature_raw(&mut self.dev);
        self.t_fine = self.calibration.temperature.compensate_temperature(adc_t as i32);
        let output = (self.t_fine * 5 + 128) >> 8;
        Ok(f64::from(output) / 100.0)
    }

    pub fn get_pressure_pascal(&mut self) -> Result<f64, String> {
        let adc_p = i2c::get_pressure_raw(&mut self.dev);
        let pressure = self.calibration.pressure.compensate_pressure(adc_p as i32, self.t_fine);
        Ok(f64::from(pressure) / 256.0)
    }

    pub fn get_humidity_relative(&mut self) -> Result<f64, String> {
        let adc_h = i2c::get_humidity_raw(&mut self.dev);
        let humidity = self.calibration.humidity.compensate_humidity(adc_h as i32, self.t_fine);

        Ok(f64::from(humidity) / 1024.0)
    }

}


#[cfg(test)]
mod tests {
    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    use super::{Address, BME280, constants::registers};

    #[test]
    fn read_humidity() {
        let address: u8 = Address::Default.into();
        let mut expectations = get_mock_calibration(address);
        expectations.push(
            I2cTransaction::write_read(address, vec![0xFD], vec![110]),    
        );
        expectations.push(
            I2cTransaction::write_read(address, vec![0xFE], vec![213]),
        );

        let i2c = I2cMock::new(&expectations);

        let mut bme280_sensor = BME280::new(i2c, address); // = BME280::build(i2c, addresses::DEFAULT);
        bme280_sensor.t_fine = 0;
        let humidity = bme280_sensor.get_humidity_relative().unwrap();
        
        assert!(humidity - 46.159 < 0.1);
    }

    #[test]
    fn read_temperature() {
        let address: u8 = Address::Default.into();
        let mut expectations = get_mock_calibration(address);
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::TEMPERATURE_MSB_REG], vec![0])
        );
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::TEMPERATURE_LSB_REG], vec![0])
        );
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::TEMPERATURE_XLSB_REG], vec![0])
        );
        
        let i2c = I2cMock::new(&expectations);

        let mut bme280_sensor = BME280::new(i2c, address);
        bme280_sensor.t_fine = 0;
        let temperature = bme280_sensor.get_temperature_celsius().unwrap();

        assert!(temperature > -100.);
        assert!(temperature < 100.);
    }

    #[test]
    fn read_pressure() {
        let address: u8 = Address::Default.into();
        let mut expectations = get_mock_calibration(address);
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::PRESSURE_MSB_REG], vec![0])
        );
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::PRESSURE_LSB_REG], vec![0])
        );
        expectations.push(
            I2cTransaction::write_read(address, vec![registers::PRESSURE_XLSB_REG], vec![0])
        );
        
        let i2c = I2cMock::new(&expectations);

        let mut bme280_sensor = BME280::new(i2c, address);
        bme280_sensor.t_fine = 0;
        let pressure = bme280_sensor.get_pressure_pascal().unwrap();

        assert!(pressure > 0.0);
    }

    fn get_mock_calibration(address: u8) -> Vec<I2cTransaction> {
        let expectations = vec![
            I2cTransaction::write_read(address, vec![registers::DIG_T1_LSB_REG], ((28485_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_T1_MSB_REG], ((28485_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // T2 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_T2_LSB_REG], ((26735_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_T2_MSB_REG], ((26735_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // T3 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_T3_LSB_REG], ((50_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_T3_MSB_REG], ((50_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),

            // Pressure calibration
            // P1 calibration
            I2cTransaction::write_read(address, vec![0x8E], ((36738_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x8F], ((36738_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P2 calibration
            I2cTransaction::write_read(address, vec![0x90], ((-10635_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x91], ((-10635_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P3 calibration
            I2cTransaction::write_read(address, vec![0x92], ((3024_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x93], ((3024_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P4 calibration
            I2cTransaction::write_read(address, vec![0x94], ((6980_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x95], ((6980_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P5 calibration
            I2cTransaction::write_read(address, vec![0x96], ((-4_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x97], ((-4_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P6 calibration
            I2cTransaction::write_read(address, vec![0x98], ((-7_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x99], ((-7_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P7 calibration
            I2cTransaction::write_read(address, vec![0x9A], ((9900_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x9B], ((9900_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P8 calibration
            I2cTransaction::write_read(address, vec![0x9C], ((-10230_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x9D], ((-10230_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),
            // P9 calibration
            I2cTransaction::write_read(address, vec![0x9E], ((4285_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![0x9F], ((4285_i64 & 0xFF00 >> 8) as u8).to_be_bytes().to_vec()),

            // TODO check all calibration values from python for sample case
            // Humidity calibration
            // H1 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H1_REG], ((75_i64 & 0xFF) as u8).to_be_bytes().to_vec()),
            // H2 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H2_LSB_REG], ((109 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_H2_MSB_REG], ((1 & 0xFF) as u8).to_be_bytes().to_vec()),
            // H3 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H3_REG], ((0 & 0xFF) as u8).to_be_bytes().to_vec()),
            // H4 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H4_MSB_REG], ((19 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_H4_LSB_REG], ((40 & 0xFF) as u8).to_be_bytes().to_vec()),
            // H5 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H5_MSB_REG], ((3 & 0xFF) as u8).to_be_bytes().to_vec()),
            I2cTransaction::write_read(address, vec![registers::DIG_H4_LSB_REG], ((40 & 0xFF) as u8).to_be_bytes().to_vec()),
            // H6 calibration
            I2cTransaction::write_read(address, vec![registers::DIG_H6_REG], ((30 & 0xFF) as u8).to_be_bytes().to_vec()),
        ];
        return expectations
    }

    #[test]
    fn random() {
        dbg!((((50_i64 & 0xFF)) as u8).to_be_bytes().to_vec());
        dbg!((((50_i64 & 0xFF00) >> 8) as u8).to_be_bytes().to_vec());
    }
}