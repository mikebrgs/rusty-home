extern crate i2cdev;
extern crate byteorder;

use std::{iter::FlatMap, collections::HashMap};

use i2cdev::{mock::MockI2CDevice, core::I2CDevice};
use byteorder::{BigEndian, LittleEndian, ByteOrder};

use hello_i2c;

fn main() {
//     println!("Creating {DEFAULT_NAME}");
//     // Creating device
//     let mut dev = MockI2CDevice::new();  // TODO change with real device
//     let _ = dev.smbus_write_byte_data(DEFAULT_ADDRESS[0], VALID_CHIP_IDS[0]);

//     // Try to write something to the device
//     dev.smbus_write_byte_data(0x0, 0).unwrap();

//     // Get chip ID
//     let chip_id = dev.smbus_read_byte_data(DEFAULT_ADDRESS[0]);
//     if !VALID_CHIP_IDS.contains(&chip_id.unwrap()) {
//         panic!("Not a valid ChipID")
//     }

//     // let mut calibration = HashMap::new();

//     // Calibration values
//     let dig_t1: Vec<u8> = smbus_read_bytes(&mut dev, &[BME280_DIG_T1_LSB_REG, BME280_DIG_T1_MSB_REG]);
//     let dig_t1 = BigEndian::read_u16(&dig_t1);

//     let dig_t2 = smbus_read_bytes(&mut dev, &[BME280_DIG_T2_LSB_REG, BME280_DIG_T2_MSB_REG]);
//     let dig_t2 = BigEndian::read_i16(&dig_t2);

//     let dig_t3 = smbus_read_bytes(&mut dev, &[BME280_DIG_T3_LSB_REG, BME280_DIG_T1_MSB_REG]);
//     let dig_t3 = BigEndian::read_i16(&dig_t3);
// }

// // Reads bytes from register addresses LSB to MSB.
// fn smbus_read_bytes(dev: &mut impl I2CDevice, registers: &[u8]) -> Vec<u8> {
//     let mut bytes_vec = vec![];

//     for register in registers {
//     // for offset in 0..bytes {
//         let value = dev.smbus_read_byte_data(*register);
//         bytes_vec.push(value.unwrap());
//     }

//     bytes_vec

    hello_i2c::BME280::new();
}

// const DEFAULT_NAME: &str = "BME280";
// const DEFAULT_ADDRESS: &[u8] = &[0x76, 0x77];
// const VALID_CHIP_IDS: &[u8] = &[0x58, 0x60];

// // mode flags for the device - user exposed
// const MODE_SLEEP: u8= 0b00;
// const MODE_FORCED: u8 = 0b01;
// const MODE_NORMAL: u8 = 0b11;

// // Register names for the BME280
// const BME280_DIG_T1_LSB_REG: u8 =         0x88;
// const BME280_DIG_T1_MSB_REG: u8 =         0x89;
// const BME280_DIG_T2_LSB_REG: u8 =         0x8A;
// const BME280_DIG_T2_MSB_REG: u8 =         0x8B;
// const BME280_DIG_T3_LSB_REG: u8 =         0x8C;
// const BME280_DIG_T3_MSB_REG: u8 =         0x8D;
// const BME280_DIG_P1_LSB_REG: u8 =         0x8E;
// const BME280_DIG_P1_MSB_REG: u8 =         0x8F;
// const BME280_DIG_P2_LSB_REG: u8 =         0x90;
// const BME280_DIG_P2_MSB_REG: u8 =         0x91;
// const BME280_DIG_P3_LSB_REG: u8 =         0x92;
// const BME280_DIG_P3_MSB_REG: u8 =         0x93;
// const BME280_DIG_P4_LSB_REG: u8 =         0x94;
// const BME280_DIG_P4_MSB_REG: u8 =         0x95;
// const BME280_DIG_P5_LSB_REG: u8 =         0x96;
// const BME280_DIG_P5_MSB_REG: u8 =         0x97;
// const BME280_DIG_P6_LSB_REG: u8 =         0x98;
// const BME280_DIG_P6_MSB_REG: u8 =         0x99;
// const BME280_DIG_P7_LSB_REG: u8 =         0x9A;
// const BME280_DIG_P7_MSB_REG: u8 =         0x9B;
// const BME280_DIG_P8_LSB_REG: u8 =         0x9C;
// const BME280_DIG_P8_MSB_REG: u8 =         0x9D;
// const BME280_DIG_P9_LSB_REG: u8 =         0x9E;
// const BME280_DIG_P9_MSB_REG: u8 =         0x9F;
// const BME280_DIG_H1_REG: u8 =             0xA1;
// const BME280_CHIP_ID_REG: u8 =            0xD0; // Chip ID
// const BME280_RST_REG: u8 =                0xE0; // Softreset Reg
// const BME280_DIG_H2_LSB_REG: u8 =         0xE1;
// const BME280_DIG_H2_MSB_REG: u8 =         0xE2;
// const BME280_DIG_H3_REG: u8 =             0xE3;
// const BME280_DIG_H4_MSB_REG: u8 =         0xE4;
// const BME280_DIG_H4_LSB_REG: u8 =         0xE5;
// const BME280_DIG_H5_MSB_REG: u8 =         0xE6;
// const BME280_DIG_H6_REG: u8 =             0xE7;
// const BME280_CTRL_HUMIDITY_REG: u8 =      0xF2; // Ctrl Humidity Reg
// const BME280_STAT_REG: u8 =               0xF3; // Status Reg
// const BME280_CTRL_MEAS_REG: u8 =          0xF4; // Ctrl Measure Reg
// const BME280_CONFIG_REG: u8 =             0xF5; // Configuration Reg
// const BME280_PRESSURE_MSB_REG: u8 =       0xF7; // Pressure MSB
// const BME280_PRESSURE_LSB_REG: u8 =       0xF8; // Pressure LSB
// const BME280_PRESSURE_XLSB_REG: u8 =      0xF9; // Pressure XLSB
// const BME280_TEMPERATURE_MSB_REG: u8 =    0xFA; // Temperature MSB
// const BME280_TEMPERATURE_LSB_REG: u8 =    0xFB; // Temperature LSB
// const BME280_TEMPERATURE_XLSB_REG: u8 =   0xFC; // Temperature XLSB
// const BME280_HUMIDITY_MSB_REG: u8 =       0xFD; // Humidity MSB
// const BME280_HUMIDITY_LSB_REG: u8 =       0xFE; // Humidity LSB