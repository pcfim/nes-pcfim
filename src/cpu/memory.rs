#[derive(PartialEq, Eq, Debug)]
pub struct Memory {
    pub memory: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; 0x10000],
        }
    }

    pub fn read_u16(&mut self, address: u16) -> u16 {
        // TODO: mutable?
        let left: u16 = self.memory[address as usize] as u16;
        let right: u16 = self.memory[address.wrapping_add(1) as usize] as u16;
        (right << 8) | left
    }

    pub fn write_u16(&mut self, address: u16, data: u16) {
        let right: u8 = (data >> 8) as u8;
        let left: u8 = (data & 0xff) as u8; // TODO: not needed?
        self.memory[address as usize] = left;
        self.memory[address.wrapping_add(1) as usize] = right;
    }

    pub fn write_u8(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
    pub fn read_u8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let start_position: usize = 0x0600;
        self.memory[start_position..(start_position + program.len())].copy_from_slice(&program[..]);
        self.write_u16(0xFFFC, start_position as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u16_correct_positions() {
        let mut memory: Memory = Memory::new();

        let address: u16 = 0x0000;
        let value: u16 = 0x04D2;
        let left: u8 = 0x04;
        let right: u8 = 0xD2;

        memory.write_u16(address, value);

        assert_eq!(left, memory.memory[address as usize + 1]);
        assert_eq!(right, memory.memory[address as usize]);
    }

    #[test]
    fn test_read_write_u16() {
        let mut memory: Memory = Memory::new();

        let address: u16 = 0x0000;
        let value: u16 = 0x04D2;

        memory.write_u16(address, value);

        assert_eq!(value, memory.read_u16(address));
    }
}
