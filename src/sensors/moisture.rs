use byteorder::{BigEndian, ByteOrder};
use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::protocols::i2c::I2CWrapper;

mod constants;
use constants::{registers, addresses};

#[derive(Debug)]
pub enum MoistureError {
    ConversionError(String),
    IOError(String)
}

pub enum Led {
    Off=0,
    On=1
}

impl From<u8> for Led {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Off,
            1 => Self::On,
            _ => panic!("Not expected")
        }
    }
}

impl From<Led> for u8 {
    fn from(item: Led) -> u8 {
        match item {
            Led::Off => 0,
            Led::On => 1,
        }
    }
}

pub enum ErrorStatus {
    Off=0,
    On=1
}

impl From<u8> for ErrorStatus {
    fn from(item: u8) -> Self {
        match item {
            0 => Self::Off,
            1 => Self::On,
            _ => panic!("Not expected")
        }
    }
}

impl From<ErrorStatus> for u8 {
    fn from(item: ErrorStatus) -> u8 {
        match item {
            ErrorStatus::Off => 0,
            ErrorStatus::On => 1,
        }
    }
}

pub enum Address {
    Default
}

impl From<Address> for u8 {
    fn from(item: Address) -> u8 {
        match item {
            Address::Default => addresses::DEFAULT
        }
    }
}

pub struct Moisture<I2C> {
    dev: I2CWrapper<I2C>
}

impl<I2C: Write + WriteRead> Moisture<I2C> {
    
    pub fn new(dev: I2C, address: u8) -> Moisture<I2C> {
        let wrapper = I2CWrapper::new(dev, address);
        Moisture { dev: wrapper }
    }

    pub fn build(dev: I2C, address: u8) -> Moisture<I2C> {
        let sensor = Moisture::new(dev, address);
        sensor
    }

    pub fn get_moisture_level(&mut self) -> Result<u16, MoistureError> {
        let mut buffer = [0u8; 2];
        let result = self.dev.read_from_register(registers::COMMAND_GET_VALUE, &mut buffer);
    
        match result {
            Ok(()) => Ok(BigEndian::read_u16(&buffer)),
            Err(_) => Err(MoistureError::IOError(String::from("Error reading moisture level.")))
        }
    }

    pub fn set_led(&mut self, led: Led) -> Result<(), MoistureError> {
        let result = self.dev.write_to_register(led.into(), &[]);
        
        match result {
            Ok(()) => Ok(()),
            Err(_) => Err(MoistureError::IOError(String::from("Error setting the LED.")))
        }
    }

    pub fn get_error_status(&mut self) -> Result<ErrorStatus, MoistureError> {
        let mut buffer = [0u8];
        let result = self.dev.read_from_register(registers::SENSOR_STATUS, &mut buffer);
    
        match result {
            Ok(()) => Ok((buffer.first().unwrap() & 0x01).into()),
            Err(_) => Err(MoistureError::IOError(String::from("Error reading error status.")))
        }

    }

    pub fn set_address(&mut self, address: u8) -> Result<(), MoistureError> {
        self.dev.write_to_register(registers::COMMAND_CHANGE_ADDRESS, &[address])
            .map_err(|_| MoistureError::IOError(String::from("Error setting address")))
    }

}


#[cfg(test)]
mod tests {
    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    use super::Moisture;

    use super::*;

    #[test]
    fn read_moisture_value() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![registers::COMMAND_GET_VALUE], vec![0x00, 0x00]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut moisture_sensor = Moisture::build(i2c, addresses::DEFAULT);
        let moisture = moisture_sensor.get_moisture_level().unwrap();

        assert_eq!(moisture, 0)
    }

    #[test]
    fn set_led_on_and_off() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write(address, vec![0x00]),
            I2cTransaction::write(address, vec![0x01]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut moisture_sensor = Moisture::build(i2c, addresses::DEFAULT);
        moisture_sensor.set_led(Led::Off).unwrap();
        moisture_sensor.set_led(Led::On).unwrap();
    }

    #[test]
    fn read_error_status() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![registers::SENSOR_STATUS], vec![0x01]),
            I2cTransaction::write(address, vec![0x01]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut moisture_sensor = Moisture::build(i2c, addresses::DEFAULT);
        let error_status: u8 = moisture_sensor.get_error_status().unwrap().into();

        assert_eq!(error_status, 1);
    }

    #[test]
    fn change_i2c_address() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write(address, vec![registers::COMMAND_CHANGE_ADDRESS, 0x50]),
        ];
        let i2c = I2cMock::new(&expectations);

        let mut moisture_sensor = Moisture::build(i2c, addresses::DEFAULT);
        moisture_sensor.set_address(0x50).unwrap();
    }


}