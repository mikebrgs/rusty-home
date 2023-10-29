use embedded_hal::blocking::i2c;

pub struct I2CWrapper<I2C> {
    i2c: I2C,
    address: u8
}

#[derive(Debug)]
pub enum I2CError{
    IOError
}

impl<I2C: i2c::Write + i2c::WriteRead> I2CWrapper<I2C> {
    pub fn new(i2c: I2C, address: u8) -> I2CWrapper<I2C> {
        I2CWrapper { address, i2c }
    }

    pub fn read_from_register(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), I2CError> {
        match self.i2c.write_read(self.address, &[register], buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err(I2CError::IOError)
        }
    }

    pub fn write_to_register(&mut self, register: u8, bytes: &[u8]) -> Result<(), I2CError> {
        let mut buffer = Vec::<u8>::with_capacity(1+bytes.len());
        buffer.push(register);
        for value in bytes {
            buffer.push(*value);
        }
        // TODO check if it matches write_bytes
        match self.i2c.write(self.address, &buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err(I2CError::IOError)
        }
    }

}


#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    use byteorder::{ByteOrder, BigEndian};
    use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    fn prepare_mock_device(expectations: &[I2cTransaction]) -> I2CWrapper<I2cMock> {
        let i2c = I2cMock::new(expectations);
        I2CWrapper::new(i2c, 0x00)

    }
    
    #[test]
    fn read_mock_i2c_register() {
        const ADDRESS: u8 = 0x00;
        const REGISTER: u8 = 0u8;

        let mut mock_buffer = vec![0u8; 2];
        BigEndian::write_u16(&mut mock_buffer, 5);

        let expectations = [
            I2cTransaction::write_read(ADDRESS, vec![REGISTER], mock_buffer.clone())
        ];

        let mut wrapper = prepare_mock_device(&expectations);

        let mut read_buffer = vec![0u8, 0u8];
        let result = wrapper.read_from_register(0u8, &mut read_buffer);
        
        // Check result
        assert_eq!(result.unwrap(), ());

        // Check output
        for x in zip(read_buffer, mock_buffer) {
            assert_eq!(x.0, x.1)
        }
    }

    #[test]
    fn write_mock_i2c_register() {
        const ADDRESS: u8 = 0x00;
        const REGISTER: u8 = 0u8;

        let mut mock_buffer = vec![0u8; 2];
        BigEndian::write_u16(&mut mock_buffer, 5);

        let expectations = [
            I2cTransaction::write(
                ADDRESS,
                {
                    let mut buffer = vec![REGISTER];
                    buffer.extend(mock_buffer.iter());
                    buffer
                }
            )
        ];

        let mut wrapper = prepare_mock_device(&expectations);

        let result = wrapper.write_to_register(0u8, &mock_buffer);
        
        // Check result
        assert_eq!(result.unwrap(), ());
    }
}