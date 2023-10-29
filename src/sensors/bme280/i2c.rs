use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{sensors::bme280::constants::{registers, values}, protocols::i2c::I2CWrapper};


pub fn read_id<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u8 {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CHIP_ID_REG, &mut buffer).unwrap();
    *buffer.first().unwrap()
}

pub fn write_reset<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) {
    dev.write_to_register(registers::RST_REG, &[values::SOFT_RESET]).unwrap();
}

pub fn read_mode<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u8 {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CTRL_MEAS_REG, &mut buffer).unwrap();
    *buffer.first().unwrap() & 0x03
}

pub fn write_mode<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, mode: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CTRL_MEAS_REG, &mut buffer).unwrap();
    let old_state = *buffer.first().unwrap() & 0xFC;
    let new_state = old_state | mode;
    dev.write_to_register(registers::CTRL_MEAS_REG, &[new_state]).unwrap();
}

pub fn read_measuring_bit<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u8 {
    // Check bit 3 is set to 1
    (read_status(dev) & 0x04) >> 2
}

pub fn read_updating_bit<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u8 {
    // Check bit 0 is set to 1
    read_status(dev) & 0x01
}

pub fn read_status<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u8 {
    let mut buffer = [0u8];
    dev.read_from_register(registers::STAT_REG, &mut buffer).unwrap();
    *buffer.first().unwrap()
}

pub fn write_humidity_oversample<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, rate: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CTRL_HUMIDITY_REG, &mut buffer).unwrap();

    let old_state = *buffer.first().unwrap() & 0xF8;
    let new_state = old_state | rate;
    dev.write_to_register(registers::CTRL_HUMIDITY_REG, &[new_state]).unwrap();
}

pub fn write_temperature_oversample<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, rate: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CTRL_MEAS_REG, &mut buffer).unwrap();

    let old_state = *buffer.first().unwrap() & 0x1F;
    let new_state = old_state | (rate << 5);
    dev.write_to_register(registers::CTRL_MEAS_REG, &[new_state]).unwrap();
}

pub fn write_pressure_oversample<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, rate: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CTRL_MEAS_REG, &mut buffer).unwrap();
    let old_state = *buffer.first().unwrap() & 0xE3;
    let new_state = old_state | (rate << 2);
    dev.write_to_register(registers::CTRL_MEAS_REG, &[new_state]).unwrap();
}

pub fn write_standby_time<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, standby: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CONFIG_REG, &mut buffer).unwrap();
    let old_state = *buffer.first().unwrap() & 0x1F;
    let new_state = old_state | (standby << 5);
    dev.write_to_register(registers::CONFIG_REG, &[new_state]).unwrap();
}

pub fn write_filter<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>, filter: u8) {
    let mut buffer = [0u8];
    dev.read_from_register(registers::CONFIG_REG, &mut buffer).unwrap();
    let old_state = *buffer.first().unwrap() & 0xE3;
    let new_state = old_state | (filter << 2);
    dev.write_to_register(registers::CONFIG_REG, &[new_state]).unwrap();
}

pub fn get_temperature_raw<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u32 {
    let mut buffer = [0u8; 3];
    dev.read_from_register(registers::TEMPERATURE_MSB_REG, &mut buffer[0..1]).unwrap();
    dev.read_from_register(registers::TEMPERATURE_LSB_REG, &mut buffer[1..2]).unwrap();
    dev.read_from_register(registers::TEMPERATURE_XLSB_REG, &mut buffer[2..3]).unwrap();

    (u32::from(buffer[0]) << 12) | (u32::from(buffer[1]) << 4) | ((u32::from(buffer[2]) >> 4) & 0x0F)
}

pub fn get_pressure_raw<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u32 {
    let mut buffer = [0u8; 3];
    dev.read_from_register(registers::PRESSURE_MSB_REG, &mut buffer[0..1]).unwrap();
    dev.read_from_register(registers::PRESSURE_LSB_REG, &mut buffer[1..2]).unwrap();
    dev.read_from_register(registers::PRESSURE_XLSB_REG, &mut buffer[2..3]).unwrap();

    (u32::from(buffer[0]) << 12) | (u32::from(buffer[1]) << 4) | ((u32::from(buffer[2]) >> 4) & 0x0F)
}

pub fn get_humidity_raw<I2C: Write + WriteRead>(dev: &mut I2CWrapper<I2C>) -> u32 {
    let mut buffer = [0u8; 2];
    dev.read_from_register(registers::HUMIDITY_MSB_REG, &mut buffer[0..1]).unwrap();
    dev.read_from_register(registers::HUMIDITY_LSB_REG, &mut buffer[1..2]).unwrap();

    (u32::from(buffer[0]) << 8) | (u32::from(buffer[1]) << 4)
}
