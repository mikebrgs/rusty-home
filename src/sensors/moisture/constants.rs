pub mod registers {
    pub const COMMAND_CHANGE_ADDRESS: u8 = 0x03;
    pub const COMMAND_GET_VALUE: u8 = 0x05;
    pub const SENSOR_STATUS: u8 = 0x3F;
}

pub mod addresses {
    pub const DEFAULT: u8 = 0x28;
}