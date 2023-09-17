use byteorder::{BigEndian, ByteOrder, LittleEndian};
use i2cdev::core::I2CDevice;
use i2cdev::mock::MockI2CDevice;

pub struct BME280 {
    dev: MockI2CDevice,
    calibration: Calibration,
    t_fine: i32,
}

impl BME280 {
    // Create new BME280 device wrapper for I2C communication.
    pub fn new() -> BME280 {
        let mut dev = MockI2CDevice::new();
        let calibration = Calibration::build(&mut dev);

        BME280{dev, calibration, t_fine: 0}
    }

    // Get temperature from the sensor.
    pub fn get_temperature_celsius(mut self) -> Result<f64, String> {
        let t1 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_MSB_REG).unwrap();
        let t2 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_LSB_REG).unwrap();
        let t3 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_XLSB_REG).unwrap();

        let adc_t = (i32::from(t1) << 12) | 
                         (i32::from(t2) << 4) |
                         ((i32::from(t3) >> 4) & 0x0F);

        let var1 = ((((adc_t>>3) - (i32::from(self.calibration.temperature.t1)<<1))) * (i32::from(self.calibration.temperature.t2))) >> 11;
        let var2 = (((((adc_t>>4) - i32::from(self.calibration.temperature.t1)) * ((adc_t>>4) - i32::from(self.calibration.temperature.t1))) >> 12) * i32::from(self.calibration.temperature.t3)) >> 14;
        self.t_fine = var1 + var2;
        let output = (self.t_fine * 5 + 128) >> 8;
        Ok(f64::from(output) / 100.0)
    }

    pub fn get_pressure_pascal(mut self) -> Result<f64, String> {
        let p1 = self.dev.smbus_read_byte_data(BME280_REGISTERS::PRESSURE_MSB_REG).unwrap();
        let p2 = self.dev.smbus_read_byte_data(BME280_REGISTERS::PRESSURE_LSB_REG).unwrap();
        let p3 = self.dev.smbus_read_byte_data(BME280_REGISTERS::PRESSURE_XLSB_REG).unwrap();

        let adc_p = (i32::from(p1) << 12) |
                         (i32::from(p2) << 4) |
                         ((i32::from(p3) >> 4) & 0x0F);

        let mut var1 = i64::from(self.t_fine) - 128000;
        let mut var2 = var1 * var1 * i64::from(self.calibration.pressure.p6);
        var2 = var2 + ((var1 * i64::from(self.calibration.pressure.p5)) << 17);
        var2 = var2 + (i64::from(self.calibration.pressure.p4) << 35);
        var1 = ((var1 * var1 * i64::from(self.calibration.pressure.p3)) >> 8) + ((var1 * i64::from(self.calibration.pressure.p2)) << 12);
        var1 = ((i64::from(1) << 47) + var1) * (i64::from(self.calibration.pressure.p1) >> 33);

        if var1 == 0 {
            return Ok(0.0)
        }

        let mut output = 1048576 - i64::from(adc_p);
        output = (((output << 31) - var2) * 3125) / var1; // Fix here
        var1 = ((i64::from(self.calibration.pressure.p9)) * (output >> 13) * (output >> 13)) >> 25;
        var2 = ((i64::from(self.calibration.pressure.p8)) * output) >> 19;
        output = ((output + var1 + var2) >> 8) + ((i64::from(self.calibration.pressure.p7)) << 4);

        let output = output as u32;  // Is this right? Sparkfun appears to be ignoring all of this
        
        Ok(f64::from(output) / (256.0 * 100.0))
    }

    pub fn get_humidity_relative(mut self) -> Result<f64, String> {
        let h1 = self.dev.smbus_read_byte_data(BME280_REGISTERS::HUMIDITY_MSB_REG).unwrap();
        let h2 = self.dev.smbus_read_byte_data(BME280_REGISTERS::HUMIDITY_LSB_REG).unwrap();

        let adc_h = (i32::from(h1) << 8) |
                         (i32::from(h2) << 4);

        let mut var1 = self.t_fine - 76800i32;
        var1 = ((((adc_h << 14) - (i32::from(self.calibration.humidity.h4) << 20) - (i32::from(self.calibration.humidity.h5) * var1)) +
            (16384)) >> 15) * (((((((var1 * i32::from(self.calibration.humidity.h6)) >> 10) * (((var1 * i32::from(self.calibration.humidity.h3)) >> 11) + (32768))) >> 10) + (2097152)) *
            i32::from(self.calibration.humidity.h2) + 8192) >> 14);
        var1 = var1 - (((((var1 >> 15) * (var1 >> 15)) >> 7) * i32::from(self.calibration.humidity.h1)) >> 4);
        if var1 < 0 {
            var1 = 0;
        } else if var1 > 419430400 {
            var1 = 419430400;
        }

        Ok(f64::from(var1>>12) / 1024.0)
    }
}

struct Calibration {
    pub temperature: TemperatureCalibration,
    pub pressure: PressureCalibration,
    pub humidity: HumidityCalibration,
}

impl Calibration {
    pub fn new(temperature: TemperatureCalibration, pressure: PressureCalibration, humidity: HumidityCalibration) -> Calibration {
        Calibration{
            temperature,
            pressure,
            humidity
        }
    }

    pub fn build(dev: &mut impl I2CDevice) -> Calibration {
        Self::new(
            TemperatureCalibration::build(dev),
            PressureCalibration::build(dev),
            HumidityCalibration::build(dev)
        )
    }
}

struct TemperatureCalibration {
    pub t1: u16,
    pub t2: i16,
    pub t3: i16
}

impl TemperatureCalibration {
    pub fn new(t1: u16, t2: i16, t3: i16) -> TemperatureCalibration {
        TemperatureCalibration{t1,t2,t3}
    }

    pub fn build(dev: &mut impl I2CDevice) -> TemperatureCalibration {
        Self::new(
            Self::read_register_t1(dev),
            Self::read_register_t2(dev),
            Self::read_register_t3(dev)
        )
    }

    fn read_register_t1(dev: &mut impl I2CDevice) -> u16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_T1_LSB_REG,
            BME280_REGISTERS::DIG_T1_MSB_REG
        ]).unwrap();
        LittleEndian::read_u16(&buffer)
    }

    fn read_register_t2(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_T2_LSB_REG,
            BME280_REGISTERS::DIG_T2_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_t3(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_T3_LSB_REG,
            BME280_REGISTERS::DIG_T3_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }
}

struct PressureCalibration {
    pub p1: u16,
    pub p2: i16,
    pub p3: i16,
    pub p4: i16,
    pub p5: i16,
    pub p6: i16,
    pub p7: i16,
    pub p8: i16,
    pub p9: i16
}

impl PressureCalibration {
    fn new(p1: u16, p2: i16, p3: i16, p4: i16, p5: i16, p6: i16, p7: i16, p8: i16, p9: i16) -> PressureCalibration {
        PressureCalibration{p1,p2,p3,p4,p5,p6,p7,p8,p9}
    }

    fn build(dev: &mut impl I2CDevice) -> PressureCalibration {
        Self::new(
            Self::read_register_p1(dev),
            Self::read_register_p2(dev),
            Self::read_register_p3(dev),
            Self::read_register_p4(dev),
            Self::read_register_p5(dev),
            Self::read_register_p6(dev),
            Self::read_register_p7(dev),
            Self::read_register_p8(dev),
            Self::read_register_p9(dev)
        )
    }

    fn read_register_p1(dev: &mut impl I2CDevice) -> u16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P1_LSB_REG,
            BME280_REGISTERS::DIG_P1_MSB_REG
        ]).unwrap();
        LittleEndian::read_u16(&buffer)
    }

    fn read_register_p2(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P2_LSB_REG,
            BME280_REGISTERS::DIG_P2_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p3(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P3_LSB_REG,
            BME280_REGISTERS::DIG_P3_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p4(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P4_LSB_REG,
            BME280_REGISTERS::DIG_P4_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p5(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P5_LSB_REG,
            BME280_REGISTERS::DIG_P5_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p6(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P6_LSB_REG,
            BME280_REGISTERS::DIG_P6_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p7(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P7_LSB_REG,
            BME280_REGISTERS::DIG_P7_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p8(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P8_LSB_REG,
            BME280_REGISTERS::DIG_P8_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_p9(dev: &mut impl I2CDevice) -> i16 {
        let buffer = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_P9_LSB_REG,
            BME280_REGISTERS::DIG_P9_MSB_REG
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }
}

struct HumidityCalibration {
    pub h1: u8,
    pub h2: i16,
    pub h3: u8,
    pub h4: i16,
    pub h5: i16,
    pub h6: i8
}

impl HumidityCalibration {
    pub fn new(h1: u8, h2: i16, h3: u8, h4: i16, h5: i16, h6: i8) -> HumidityCalibration {
        HumidityCalibration{h1,h2,h3,h4,h5,h6}
    }

    pub fn build(dev: &mut impl I2CDevice) -> HumidityCalibration {
        Self::new(
            Self::read_register_h1(dev),
            Self::read_register_h2(dev),
            Self::read_register_h3(dev),
            Self::read_register_h4(dev),
            Self::read_register_h5(dev),
            Self::read_register_h6(dev)
        )
    }

    fn read_register_h1(dev: &mut impl I2CDevice) -> u8 {
        let mut buffer = read_multiple_registers(dev, &[BME280_REGISTERS::DIG_H1_REG]).unwrap();
        buffer.pop().unwrap()
    }

    fn read_register_h2(dev: &mut impl I2CDevice) -> i16 {
        let buffer: Vec<u8> = read_multiple_registers(dev, &[
            BME280_REGISTERS::DIG_H2_LSB_REG,
            BME280_REGISTERS::DIG_H2_MSB_REG,
        ]).unwrap();
        LittleEndian::read_i16(&buffer)
    }

    fn read_register_h3(dev: &mut impl I2CDevice) -> u8 {
        let mut buffer = read_multiple_registers(dev, &[BME280_REGISTERS::DIG_H3_REG]).unwrap();
        buffer.pop().unwrap()
    }

    fn read_register_h4(dev: &mut impl I2CDevice) -> i16 {
        let h4_msb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_H4_MSB_REG).unwrap();
        let h4_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_H4_LSB_REG).unwrap();
        ((u16::from(h4_msb) << 4) | (u16::from(h4_lsb) & 0x0F)) as i16
    }

    fn read_register_h5(dev: &mut impl I2CDevice) -> i16 {
        let h4_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_H4_LSB_REG).unwrap();
        let h5_msb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_H5_MSB_REG).unwrap();
        (((u16::from(h5_msb) << 4)) | ((u16::from(h4_lsb) >> 4) & 0x0F)) as i16
    }

    fn read_register_h6(dev: &mut impl I2CDevice) -> i8 {
        let h4_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_H6_REG).unwrap();
        h4_lsb as i8
    }
}

fn read_multiple_registers(dev: &mut impl I2CDevice, registers: &[u8]) -> Result<Vec<u8>, ()> {
    let mut buffer = vec![];
    for register in registers.iter() {
        match dev.smbus_read_byte_data(*register) {
            Ok(byte) => buffer.push(byte),
            Err(_) => {return Err(())}
        }
    }
    Ok(buffer)
}

mod BME280_REGISTERS {
    pub const DIG_T1_LSB_REG: u8 = 0x88;
    pub const DIG_T1_MSB_REG: u8 = 0x89;
    pub const DIG_T2_LSB_REG: u8 = 0x8A;
    pub const DIG_T2_MSB_REG: u8 = 0x8B;
    pub const DIG_T3_LSB_REG: u8 = 0x8C;
    pub const DIG_T3_MSB_REG: u8 = 0x8D;

    pub const DIG_P1_LSB_REG: u8 = 0x8E;
    pub const DIG_P1_MSB_REG: u8 = 0x8F;
    pub const DIG_P2_LSB_REG: u8 = 0x90;
    pub const DIG_P2_MSB_REG: u8 = 0x91;
    pub const DIG_P3_LSB_REG: u8 = 0x92;
    pub const DIG_P3_MSB_REG: u8 = 0x93;
    pub const DIG_P4_LSB_REG: u8 = 0x94;
    pub const DIG_P4_MSB_REG: u8 = 0x95;
    pub const DIG_P5_LSB_REG: u8 = 0x96;
    pub const DIG_P5_MSB_REG: u8 = 0x97;
    pub const DIG_P6_LSB_REG: u8 = 0x98;
    pub const DIG_P6_MSB_REG: u8 = 0x99;
    pub const DIG_P7_LSB_REG: u8 = 0x9A;
    pub const DIG_P7_MSB_REG: u8 = 0x9B;
    pub const DIG_P8_LSB_REG: u8 = 0x9C;
    pub const DIG_P8_MSB_REG: u8 = 0x9D;
    pub const DIG_P9_LSB_REG: u8 = 0x9E;
    pub const DIG_P9_MSB_REG: u8 = 0x9F;

    pub const DIG_H1_REG: u8 = 0xA1;
    pub const DIG_H2_LSB_REG: u8 = 0xE1;
    pub const DIG_H2_MSB_REG: u8 = 0xE2;
    pub const DIG_H3_REG: u8 = 0xE3;
    pub const DIG_H4_MSB_REG: u8 = 0xE4;
    pub const DIG_H4_LSB_REG: u8 = 0xE5;
    pub const DIG_H5_MSB_REG: u8 = 0xE6;
    pub const DIG_H6_REG: u8 = 0xE7;

    pub const TEMPERATURE_MSB_REG: u8 = 0xFA;  // Temperature MSB
    pub const TEMPERATURE_LSB_REG: u8 = 0xFB;  // Temperature LSB
    pub const TEMPERATURE_XLSB_REG: u8 = 0xFC;  // Temperature XLSB

    pub const PRESSURE_MSB_REG: u8 = 0xF7;  // Pressure MSB
    pub const PRESSURE_LSB_REG: u8 = 0xF8;  // Pressure LSB
    pub const PRESSURE_XLSB_REG: u8 = 0xF9;  // Pressure XLSB

    pub const HUMIDITY_MSB_REG: u8 = 0xFD;  // Humidity MSB
    pub const HUMIDITY_LSB_REG: u8 = 0xFE;  // Humidity LSB
}
