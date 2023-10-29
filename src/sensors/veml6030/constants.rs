pub mod registers {
    pub const SETTING_REG : u8 = 0x00;
    pub const H_THRESH_REG : u8 = 0x01;
    pub const L_THRESH_REG : u8 = 0x02;
    pub const POWER_SAVE_REG : u8 = 0x03;
    pub const AMBIENT_LIGHT_DATA_REG : u8 = 0x04;
    pub const WHITE_LIGHT_DATA_REG : u8 = 0x05;
    pub const INTERRUPT_REG : u8 = 0x06;
}

pub mod addresses {
    pub const DEFAULT: u8 = 0x48;
    pub const ALTERNATIVE: u8 = 0x10;
}