use bitflags::bitflags;

bitflags! {
    // https://wiki.nesdev.com/w/index.php/Controller_reading_code
    pub struct JoypadButton: u8 {
        const RIGHT             = 0b10000000;
        const LEFT              = 0b01000000;
        const DOWN              = 0b00100000;
        const UP                = 0b00010000;
        const START             = 0b00001000;
        const SELECT            = 0b00000100;
        const BUTTON_B          = 0b00000010;
        const BUTTON_A          = 0b00000001;
    }
}

pub struct Joypad {
    pub strobe: bool,
    pub button_index: u8,
    pub button_status: JoypadButton,
}
