use byteorder::{ByteOrder, BigEndian};
use embedded_hal::blocking::i2c::{Write, WriteRead};

mod constants;

use crate::protocols::i2c::{self, I2CWrapper};

use constants::registers;

use self::constants::addresses;

#[derive(Debug)]
pub enum VEML6030Error {
    ConversionError,
    IOError
}

pub enum Gain {
    X1 = 0b00,  // 1x gain
    X2 = 0b01,  // 2x gain
    X1_8 = 0b10,  // 1/8x gain
    X1_4 = 0b11  // 1/4x gain
}

impl From<u16> for Gain {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::X1,
            1 => Self::X2,
            2 => Self::X1_8,
            3 => Self::X1_4,
            _ => panic!("Not expected")
        }
    }
}

impl From<Gain> for u16 {
    fn from(item: Gain) -> u16 {
        match item {
            Gain::X1 => 0,
            Gain::X2 => 1,
            Gain::X1_8 => 2,
            Gain::X1_4 => 3,
        }
    }
}

pub enum IntegrationTime {
    Ms25 = 0b1100,
    Ms50 = 0b1000,
    Ms100 = 0b0000,
    Ms200 = 0b0001,
    Ms400 = 0b0010,
    Ms800 = 0b0011
}

impl From<u16> for IntegrationTime {
    fn from(item: u16) -> Self {
        match item {
            0b1100 => Self::Ms25,
            0b1000 => Self::Ms50,
            0b0000 => Self::Ms100,
            0b0001 => Self::Ms200,
            0b0010 => Self::Ms400,
            0b0011 => Self::Ms800,
            _ => panic!("Not expected")
        }
    }
}

impl From<IntegrationTime> for u16 {
    fn from(item: IntegrationTime) -> u16 {
        match item {
            IntegrationTime::Ms25 => 0b1100,
            IntegrationTime::Ms50 => 0b1000,
            IntegrationTime::Ms100 => 0b0000,
            IntegrationTime::Ms200 => 0b0001,
            IntegrationTime::Ms400 => 0b0010,
            IntegrationTime::Ms800 => 0b0011,
        }
    }
}

pub enum PersistenceProtectNumber {
    N1 = 0b00,
    N2 = 0b01,
    N4 = 0b10,
    N8 = 0b11
}

impl From<u16> for PersistenceProtectNumber {
    fn from(item: u16) -> Self {
        match item {
            0b00 => Self::N1,
            0b01 => Self::N2,
            0b10 => Self::N4,
            0b11 => Self::N8,
            _ => panic!("Not expected")
        }
    }
}

impl From<PersistenceProtectNumber> for u16 {
    fn from(item: PersistenceProtectNumber) -> u16 {
        match item {
            PersistenceProtectNumber::N1 => 0b00,
            PersistenceProtectNumber::N2 => 0b01,
            PersistenceProtectNumber::N4 => 0b10,
            PersistenceProtectNumber::N8 => 0b11,
        }
    }
}

pub enum PowerSavingMode {
    M1 = 0b00,  // Fastest, most current
    M2 = 0b01,
    M3 = 0b10,
    M4 = 0b11  // Slowest, least current
}

impl From<u16> for PowerSavingMode {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::M1,
            1 => Self::M2,
            2 => Self::M3,
            3 => Self::M4,
            _ => panic!("Not expected")
        }
    }
}

impl From<PowerSavingMode> for u16 {
    fn from(item: PowerSavingMode) -> u16 {
        match item {
            PowerSavingMode::M1 => 0,
            PowerSavingMode::M2 => 1,
            PowerSavingMode::M3 => 2,
            PowerSavingMode::M4 => 3,
        }
    }
}

pub enum PowerSavingModeEnable {
    Disable = 0b0,
    Enable = 0b1
}

impl From<u16> for PowerSavingModeEnable {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::Disable,
            1 => Self::Enable,
            _ => panic!("Not expected")
        }
    }
}

impl From<PowerSavingModeEnable> for u16 {
    fn from(item: PowerSavingModeEnable) -> u16 {
        match item {
            PowerSavingModeEnable::Disable => 0,
            PowerSavingModeEnable::Enable => 1,
        }
    }
}


pub enum InterruptEnable {
    Disable = 0b0,
    Enable = 0b1,
}

impl From<u16> for InterruptEnable {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::Disable,
            1 => Self::Enable,
            _ => panic!("Not expected")
        }
    }
}

impl From<InterruptEnable> for u16 {
    fn from(item: InterruptEnable) -> u16 {
        match item {
            InterruptEnable::Disable => 0,
            InterruptEnable::Enable => 1,
        }
    }
}

pub enum Shutdown {
    PowerOn,
    PowerOff
}

impl From<u16> for Shutdown {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::PowerOn,
            1 => Self::PowerOff,
            _ => panic!("Not expected")
        }
    }
}

impl From<Shutdown> for u16 {
    fn from(item: Shutdown) -> u16 {
        match item {
            Shutdown::PowerOn => 0,
            Shutdown::PowerOff => 1,
        }
    }
}

pub enum Threshold {
    NotExceeded = 0,
    Exceeded = 1,
}

impl From<u16> for Threshold {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::NotExceeded,
            1 => Self::Exceeded,
            _ => panic!("Not expected")
        }
    }
}

impl From<Threshold> for u16 {
    fn from(item: Threshold) -> u16 {
        match item {
            Threshold::NotExceeded => 0,
            Threshold::Exceeded => 1,
        }
    }
}

pub enum Address {
    Default,
    Alternative,
}

impl From<Address> for u8 {
    fn from(item: Address) -> u8 {
        match item {
            Address::Default => addresses::DEFAULT,
            Address::Alternative => addresses::ALTERNATIVE,
        }
    }
}

pub struct VEML6030<I2C> {
    dev: i2c::I2CWrapper<I2C>
}

impl<I2C: Write + WriteRead> VEML6030<I2C> {
    pub fn new(dev: I2C, address: u8) -> Self {
        let i2c_wrapper = I2CWrapper::new(dev, address);
        VEML6030{dev: i2c_wrapper}
    }

    pub fn build(dev: I2C, address: u8) -> VEML6030<I2C> {
        let mut sensor = Self::new(dev, address);

        sensor.set_shutdown(Shutdown::PowerOn).unwrap();
        sensor.set_gain(Gain::X1_4).unwrap();
        sensor.set_integration_time(IntegrationTime::Ms50).unwrap();

        sensor
    }

    pub fn get_gain(&mut self) -> Result<Gain, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let gain = clip_u16(state, 11, 2);

        Ok(gain.into())
    }

    pub fn set_gain(&mut self, gain: Gain) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 11, 2, gain.into());
        self.convert_and_write_u16(registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_integration_time(&mut self) -> Result<IntegrationTime, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let integration_time = clip_u16(state, 6, 4);

        Ok(integration_time.into())
    }

    pub fn set_integration_time(&mut self, integration_time: IntegrationTime) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 6, 4, integration_time.into());
        self.convert_and_write_u16(registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_persist_protect_number(&mut self) -> Result<PersistenceProtectNumber, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let ppn = clip_u16(state, 4, 2);
        
        Ok(ppn.into())
    }

    pub fn set_persist_protect_number(&mut self, persistence_protect_number: PersistenceProtectNumber) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 4, 2, persistence_protect_number.into());
        self.convert_and_write_u16(registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_interrupt_enabled(&mut self) -> Result<InterruptEnable, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let interrupt = clip_u16(state, 1, 1);

        Ok(interrupt.into())
    }

    pub fn set_interrupt_enabled(&mut self, interrupt_enable: InterruptEnable) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let new_sate = insert_u16(old_state, 1, 1, interrupt_enable.into());
        self.convert_and_write_u16(registers::SETTING_REG, new_sate).unwrap();
        
        Ok(())
    }

    pub fn get_shutdown(&mut self) -> Result<Shutdown, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let shut_down = clip_u16(state, 0, 1);

        Ok(shut_down.into())
    }

    pub fn set_shutdown(&mut self, shutdown: Shutdown) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::SETTING_REG).unwrap();
        let new_sate = insert_u16(old_state, 1, 1, shutdown.into());
        self.convert_and_write_u16(registers::SETTING_REG, new_sate).unwrap();
        
        Ok(())
    }

    pub fn get_high_threshold_window(&mut self) -> Result<u16, VEML6030Error> {
        let high_threhold_window = self.read_and_convert_to_u16(registers::H_THRESH_REG).unwrap();

        Ok(high_threhold_window)
    }

    pub fn set_high_threshold_window(&mut self, threshold: u16) -> Result<(), VEML6030Error> {
        self.convert_and_write_u16(registers::H_THRESH_REG, threshold).unwrap();

        Ok(())
    }

    pub fn get_low_threshold_window(&mut self) -> Result<u16, VEML6030Error> {
        let low_threshold_window = self.read_and_convert_to_u16(registers::L_THRESH_REG).unwrap();

        Ok(low_threshold_window)
    }

    pub fn set_low_threshold_window(&mut self, threshold: u16) -> Result<(), VEML6030Error> {
        self.convert_and_write_u16(registers::L_THRESH_REG, threshold).unwrap();

        Ok(())
    }

    pub fn get_power_saving_mode(&mut self) -> Result<PowerSavingMode, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::POWER_SAVE_REG).unwrap();
        let mode = clip_u16(state, 1, 2);
        
        Ok(mode.into())
    }

    pub fn set_power_saving_mode(&mut self, mode: PowerSavingMode) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::POWER_SAVE_REG).unwrap();
        let new_state = insert_u16(old_state, 1, 2, mode.into());
        self.convert_and_write_u16(registers::POWER_SAVE_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_power_saving_mode_enabled(&mut self) -> Result<PowerSavingModeEnable, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::POWER_SAVE_REG).unwrap();
        let enabled = clip_u16(state, 0, 1);

        Ok(enabled.into())
    }

    pub fn set_power_saving_mode_enabled(&mut self, enable: PowerSavingModeEnable) -> Result<(), VEML6030Error> {
        let old_state = self.read_and_convert_to_u16(registers::POWER_SAVE_REG).unwrap();
        let new_state = insert_u16(old_state, 0, 1, enable.into());
        self.convert_and_write_u16(registers::POWER_SAVE_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_ambient_light_output(&mut self) -> Result<u16, VEML6030Error> {
        let alo = self.read_and_convert_to_u16(registers::AMBIENT_LIGHT_DATA_REG).unwrap();

        Ok(alo)
    } 

    pub fn get_white_light_output(&mut self) -> Result<u16, VEML6030Error> {
        let wlo = self.read_and_convert_to_u16(registers::WHITE_LIGHT_DATA_REG).unwrap();
        
        Ok(wlo)
    }

    pub fn get_low_threshold_exceeded(&mut self) -> Result<Threshold, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::INTERRUPT_REG).unwrap();
        let threshold_exceeded = clip_u16(state, 15, 1);
        
        Ok(threshold_exceeded.into())
    }

    pub fn get_high_threshold_exceeded(&mut self) -> Result<Threshold, VEML6030Error> {
        let state = self.read_and_convert_to_u16(registers::INTERRUPT_REG).unwrap();
        let threshold_exceeded = clip_u16(state, 14, 1);
        
        Ok(threshold_exceeded.into())
    }

    pub fn convert_to_lux(&mut self, raw: u16) -> Result<f32, VEML6030Error> {
        const LX_BIT: f32 = 0.0288;

        let gain = self.get_gain().unwrap();
        let integration_time = self.get_integration_time().unwrap();

        let float_raw: f32 = raw.into();
        let mut lux = match integration_time {
            IntegrationTime::Ms25 => float_raw * LX_BIT * 4.0,
            IntegrationTime::Ms50 => float_raw * LX_BIT * 2.0,
            IntegrationTime::Ms100 => float_raw * LX_BIT,
            IntegrationTime::Ms200 => float_raw * LX_BIT / 2.0,
            IntegrationTime::Ms400 => float_raw * LX_BIT / 4.0,
            IntegrationTime::Ms800 => float_raw * LX_BIT / 8.0
        };

        lux = match gain {
            Gain::X1 => lux * 2.0,
            Gain::X2 => lux,
            Gain::X1_4 => lux * 8.0,
            Gain::X1_8 => lux * 16.0
        };

        Ok(lux)
    }

    pub fn convert_from_lux(&mut self, lux: f32) -> Result<u16, VEML6030Error> {
        const LX_BIT: f32 = 0.0288;
        let gain = self.get_gain().unwrap();
        let integration_time = self.get_integration_time().unwrap();

        let mut lux = match gain {
            Gain::X1 => lux / 2.0,
            Gain::X2 => lux,
            Gain::X1_4 => lux / 8.0,
            Gain::X1_8 => lux / 16.0
        };

        lux = match integration_time {
            IntegrationTime::Ms25 => lux / (LX_BIT * 4.0),
            IntegrationTime::Ms50 => lux / (LX_BIT * 2.0),
            IntegrationTime::Ms100 => lux / (LX_BIT),
            IntegrationTime::Ms200 => lux / (LX_BIT * 2.0),
            IntegrationTime::Ms400 => lux / (LX_BIT * 4.0),
            IntegrationTime::Ms800 => lux / (LX_BIT * 8.0)
        };

        Ok(lux as u16)

    }

    pub fn compensate_lux(&mut self, lux: f32) -> Result<f32, VEML6030Error> {
        if lux > 1000. {
            return Ok(0.00000000000060135 * lux.powi(4)
                - 0.0000000093924 * lux.powi(3)
                + 0.000081488 * lux.powi(2)
                + 1.0023 * lux)
        }
        Ok(lux)
    }

    pub fn get_ambient_light_lux(&mut self) -> Result<f32, VEML6030Error> {
        let raw_lux = self.get_ambient_light_output().unwrap();
        let lux = self.convert_to_lux(raw_lux).unwrap();
        let lux = self.compensate_lux(lux).unwrap();

        Ok(lux)
    }

    pub fn get_white_light_lux(&mut self) -> Result<f32, VEML6030Error> {
        let raw_lux = self.get_white_light_output().unwrap();
        let lux = self.convert_to_lux(raw_lux).unwrap();
        let lux = self.compensate_lux(lux).unwrap();

        Ok(lux)
    }

    // BREAK to Common methods

    fn read_and_convert_to_u16(&mut self, register: u8) -> Result<u16, VEML6030Error> {
        let mut buffer = [0u8; 2];
        self.dev.read_from_register(register, &mut buffer).unwrap();
        let state = convert_buffer_to_u16(&buffer).unwrap();
        Ok(state)
    }

    fn convert_and_write_u16(&mut self, register: u8, state: u16) -> Result<(), VEML6030Error> {
        let mut buffer = [0u8; 2];
        convert_u16_to_buffer(&mut buffer, state).unwrap();
        self.dev.write_to_register(register, &buffer).unwrap();
        Ok(())
    }

}

fn convert_buffer_to_u16(buffer: &[u8]) -> Result<u16, ()> {
    let num = BigEndian::read_u16(buffer);
    Ok(num)
}

fn convert_u16_to_buffer(buffer: &mut [u8], num: u16) -> Result<(), ()> {
    BigEndian::write_u16(buffer, num);
    Ok(())
}

fn clip_u16(state: u16, trailing_zeros: u16, length: u16) -> u16 {
    let mask = create_mask(trailing_zeros, length);
    (state & mask) >> trailing_zeros
}

fn insert_u16(state: u16, trailing_zeros: u16, length: u16, value: u16) -> u16 {
    let mask = create_mask(trailing_zeros, length);
    (state & !mask) | ((value << trailing_zeros) & mask)
}

fn create_mask(trailing_zeros: u16, length: u16) -> u16 {
    let mut mask = 0u16;
    for _ in 0..length {
        mask = (mask << 1) + 1u16
    }
    mask << trailing_zeros

}


#[cfg(test)]
mod tests {
    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    use crate::sensors::veml6030::constants::addresses;

    use super::*;

    #[test]
    fn start_veml6030() {
        let address: u8 = Address::Default.into();
        let expectations = [
            I2cTransaction::write_read(address, vec![registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![registers::SETTING_REG, 0x00, 0x00]),
            I2cTransaction::write_read(address, vec![registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![registers::SETTING_REG, 0x18, 0x00]),
            I2cTransaction::write_read(address, vec![registers::SETTING_REG], vec![0x18, 0x00]),
            I2cTransaction::write(address, vec![registers::SETTING_REG, 0x1A, 0x00]),
        ];
        let i2c = I2cMock::new(&expectations);

        let _veml6030 = VEML6030::build(i2c, addresses::DEFAULT);
    }

}