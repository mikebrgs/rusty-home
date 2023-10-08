use i2cdev::core::I2CDevice;
use crate::sensors::bme280::constants::{registers, values};


pub fn read_id(dev: &mut impl I2CDevice) -> u8 {
    dev.smbus_read_byte_data(registers::CHIP_ID_REG).unwrap()
}

pub fn write_reset(dev: &mut impl I2CDevice) {
    dev.smbus_write_byte_data(registers::RST_REG, values::SOFT_RESET).unwrap();
}

pub fn read_mode(dev: &mut impl I2CDevice) -> u8 {
    let control_data: u8 = dev.smbus_read_byte_data(registers::CTRL_MEAS_REG).unwrap();
    control_data & 0x03
}

pub fn write_mode(dev: &mut impl I2CDevice, mode: u8) {
    let mut control_data = dev.smbus_read_byte_data(registers::CTRL_MEAS_REG).unwrap();
    control_data = control_data & 0xFC;
    control_data = control_data | mode;
    dev.smbus_write_byte_data(registers::CTRL_MEAS_REG, control_data).unwrap();
}

pub fn read_measuring_bit(dev: &mut impl I2CDevice) -> u8 {
    // Check bit 3 is set to 1
    (read_status(dev) & 0x04) >> 2
}

pub fn read_updating_bit(dev: &mut impl I2CDevice) -> u8 {
    // Check bit 0 is set to 1
    read_status(dev) & 0x01
}

pub fn read_status(dev: &mut impl I2CDevice) -> u8 {
    dev.smbus_read_byte_data(registers::STAT_REG).unwrap()
}

pub fn write_humidity_oversample(dev: &mut impl I2CDevice, rate: u8) {
    let mut control_humidity = dev.smbus_read_byte_data(registers::CTRL_HUMIDITY_REG).unwrap();
    control_humidity = control_humidity & 0xF8;
    control_humidity = control_humidity | rate;
    dev.smbus_write_byte_data(registers::CTRL_HUMIDITY_REG, control_humidity).unwrap();
}

pub fn write_temperature_oversample(dev: &mut impl I2CDevice, rate: u8) {
    let mut control_temperature = dev.smbus_read_byte_data(registers::CTRL_MEAS_REG).unwrap();
    control_temperature = control_temperature & 0x1F;
    control_temperature = control_temperature | (rate << 5);
    dev.smbus_write_byte_data(registers::CTRL_MEAS_REG, control_temperature).unwrap();
}

pub fn write_pressure_oversample(dev: &mut impl I2CDevice, rate: u8) {
    let mut control_pressure = dev.smbus_read_byte_data(registers::CTRL_MEAS_REG).unwrap();
    control_pressure = control_pressure & 0xE3;
    control_pressure = control_pressure | (rate << 2);
    dev.smbus_write_byte_data(registers::CTRL_MEAS_REG, control_pressure).unwrap();
}

pub fn write_standby_time(dev: &mut impl I2CDevice, standby: u8) {
    let mut standby_control = dev.smbus_read_byte_data(registers::CONFIG_REG).unwrap();
    standby_control = standby_control & 0x1F;
    standby_control = standby_control | (standby << 5);
    dev.smbus_write_byte_data(registers::CONFIG_REG, standby_control).unwrap();
}

pub fn write_filter(dev: &mut impl I2CDevice, filter: u8) {
    let mut filter_control = dev.smbus_read_byte_data(registers::CONFIG_REG).unwrap();
    filter_control = filter_control & 0xE3;
    filter_control = filter_control | (filter << 2);
    dev.smbus_write_byte_data(registers::CONFIG_REG, filter_control).unwrap();
}

pub fn get_temperature_raw(dev: &mut impl I2CDevice) -> u32 {
    let t1 = dev.smbus_read_byte_data(registers::TEMPERATURE_MSB_REG).unwrap();
    let t2 = dev.smbus_read_byte_data(registers::TEMPERATURE_LSB_REG).unwrap();
    let t3 = dev.smbus_read_byte_data(registers::TEMPERATURE_XLSB_REG).unwrap();

    (u32::from(t1) << 12) | (u32::from(t2) << 4) | ((u32::from(t3) >> 4) & 0x0F)
}

pub fn get_pressure_raw(dev: &mut impl I2CDevice) -> u32 {
    let p1 = dev.smbus_read_byte_data(registers::PRESSURE_MSB_REG).unwrap();
    let p2 = dev.smbus_read_byte_data(registers::PRESSURE_LSB_REG).unwrap();
    let p3 = dev.smbus_read_byte_data(registers::PRESSURE_XLSB_REG).unwrap();

    (u32::from(p1) << 12) | (u32::from(p2) << 4) | ((u32::from(p3) >> 4) & 0x0F)
}

pub fn get_humidity_raw(dev: &mut impl I2CDevice) -> u32 {
    let h1 = dev.smbus_read_byte_data(registers::HUMIDITY_MSB_REG).unwrap();
    let h2 = dev.smbus_read_byte_data(registers::HUMIDITY_LSB_REG).unwrap();

    (u32::from(h1) << 8) | (u32::from(h2) << 4)
}
