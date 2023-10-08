use std::u8;

use i2cdev::core::I2CDevice;

pub mod calibration;
pub mod i2c;
pub mod constants;

use constants::values;

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
    T05,
    T625,
    T1250,
    T2500,
    T5000,
    T10000,
    T100,
    T200
}

impl From<u8> for StandyTime {
    fn from(value: u8) -> Self {
        match value {
            0 => StandyTime::T05,
            1 => StandyTime::T625,
            2 => StandyTime::T1250,
            3 => StandyTime::T2500,
            4 => StandyTime::T5000,
            5 => StandyTime::T10000,
            6 => StandyTime::T100,
            7 => StandyTime::T200,
            _ => panic!("Invalid standby value")
        }
    }
}

impl From<StandyTime> for u8 {
    fn from(value: StandyTime) -> Self {
        match value {
             StandyTime::T05 => 0,
             StandyTime::T625 => 1,
             StandyTime::T1250 => 2,
             StandyTime::T2500 => 3,
             StandyTime::T5000 => 4,
             StandyTime::T10000 => 5,
             StandyTime::T100 => 6,
             StandyTime::T200 => 7
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

pub struct BME280<I2C> {
    dev: I2C,
    calibration: calibration::Calibration,
    t_fine: i32,
}

impl<I2C: I2CDevice> BME280<I2C> {

    // Create new BME280 device wrapper for I2C communication.
    pub fn new(mut dev: I2C) -> BME280<I2C> {
        let calibration = calibration::Calibration::build(&mut dev);
        BME280 { dev: dev, calibration: calibration, t_fine: 0 }
    }

    // Start all parameters from for the sensor
    pub fn start(&mut self) -> Result<(), String> {
        self.set_standby_time(StandyTime::T05).unwrap();
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
