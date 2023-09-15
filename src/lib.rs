use byteorder::{BigEndian, ByteOrder};
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
    pub fn get_temperature_celsius(mut self) -> f64 {
        let t1 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_MSB_REG).unwrap();
        let t2 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_LSB_REG).unwrap();
        let t3 = self.dev.smbus_read_byte_data(BME280_REGISTERS::TEMPERATURE_XLSB_REG).unwrap();
        let data_buffer = [t1, t2, t3];

        let adc_t = (i32::from(data_buffer[0]) << 12) | 
                         (i32::from(data_buffer[1]) << 4) |
                         ((i32::from(data_buffer[2]) >> 4) & 0x0F);

        let var1 = ((((adc_t>>3) - (i32::from(self.calibration.t1)<<1))) * (i32::from(self.calibration.t2))) >> 11;
        let var2 = (((((adc_t>>4) - i32::from(self.calibration.t1)) * ((adc_t>>4) - i32::from(self.calibration.t1))) >> 12) * i32::from(self.calibration.t3)) >> 14;
        self.t_fine = var1 + var2;
        let output = (self.t_fine * 5 + 128) >> 8;
        f64::from(output) / 100.0
    }
}

mod BME280_REGISTERS {
    pub const DIG_T1_LSB_REG: u8 = 0x88;
    pub const DIG_T1_MSB_REG: u8 = 0x89;
    pub const DIG_T2_LSB_REG: u8 = 0x8A;
    pub const DIG_T2_MSB_REG: u8 = 0x8B;
    pub const DIG_T3_LSB_REG: u8 = 0x8C;
    pub const DIG_T3_MSB_REG: u8 = 0x8D;

    pub const TEMPERATURE_MSB_REG: u8 =    0xFA; // Temperature MSB
    pub const TEMPERATURE_LSB_REG: u8 =    0xFB; // Temperature LSB
    pub const TEMPERATURE_XLSB_REG: u8 =   0xFC; // Temperature XLSB
    
}

struct Calibration {
    pub t1: u16,
    pub t2: i16,
    pub t3: i16,
}

impl Calibration {
    pub fn new(t1: u16, t2: i16, t3: i16) -> Calibration {
        Calibration{t1, t2, t3}
    }
    pub fn build(dev: &mut impl I2CDevice) -> Calibration {
        let t1_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T1_LSB_REG).unwrap();
        let t1_msb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T1_MSB_REG).unwrap();
        let t1 = BigEndian::read_u16(&[t1_lsb, t1_msb]);

        let t2_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T2_LSB_REG).unwrap();
        let t2_msb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T2_MSB_REG).unwrap();
        let t2 = BigEndian::read_i16(&[t2_lsb, t2_msb]);

        let t3_lsb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T3_LSB_REG).unwrap();
        let t3_msb = dev.smbus_read_byte_data(BME280_REGISTERS::DIG_T3_MSB_REG).unwrap();
        let t3 = BigEndian::read_i16(&[t3_lsb, t3_msb]);

        Calibration::new(t1, t2, t3)
    }
}