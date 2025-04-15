pub struct AddressRegister {
    value: (u8, u8),
    high_ptr: bool,
}

impl Default for AddressRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            value: (0, 0),
            high_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    fn mirror_down(&mut self) {
        if self.get() > 0x3fff {
            // Mirror down after 0x3fff
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_latch(&mut self) {
        self.high_ptr = true;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    pub fn update(&mut self, data: u8) {
        if self.high_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }
        self.mirror_down();
        self.high_ptr = !self.high_ptr;
    }

    pub fn increment(&mut self, increment: u8) {
        /*
         * Alternative implementation (?):
         * let current_value: u16 = self.get();
         * current_value = current_value.wrapping_add(increment);
         * self.set();
         * self.mirror_down();
         */
        let previous_low_value: u8 = self.value.1;
        self.value.1 = self.value.1.wrapping_add(increment);
        if previous_low_value > self.value.1 {
            // overflow low bits, then increment the higher ones
            self.value.0 = self.value.0.wrapping_add(1);
        }
        self.mirror_down();
    }
}
