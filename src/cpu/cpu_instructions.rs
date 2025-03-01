use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::memory::Memory;
use std::collections::HashMap;

use super::operation_codes;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: Memory,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: Memory::new(),
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.memory.memory[self.program_counter as usize] as u16,

            AddressingMode::Absolute => self.memory.read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.memory.memory[self.program_counter as usize];
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.memory.memory[self.program_counter as usize];
                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute_X => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.memory.memory[self.program_counter as usize];

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.memory.memory[ptr as usize];
                let hi = self.memory.memory[ptr.wrapping_add(1) as usize];
                ((hi as u16) << 8) | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.memory.memory[self.program_counter as usize];

                let lo = self.memory.memory[base as usize];
                let hi = self.memory.memory[base.wrapping_add(1) as usize];
                let deref_base = ((hi as u16) << 8) | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn load_accumulator(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value: u8 = self.memory.memory[address as usize];

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn transfer_accumulator_to_x(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn store_accumulator(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        self.memory.memory[address as usize] = self.register_a;
    }

    fn increment_x_register(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn compare(&mut self, mode: &AddressingMode, value_to_compare: u8) {
        let address = self.get_operand_address(mode);
        let value: u8 = self.memory.memory[address as usize];

        if value_to_compare >= value {
            self.status |= 0b0000_00001;
        }

        self.update_zero_and_negative_flags(value_to_compare.wrapping_sub(value));
    }

    pub fn run(&mut self) {
        let operation_codes: &HashMap<u8, &'static operation_codes::Operation> =
            &operation_codes::OPERATION_CODES_MAP;
        loop {
            let code = self.memory.memory[self.program_counter as usize];
            self.program_counter += 1;
            let program_counter_previous = self.program_counter;
            let operation_code = operation_codes
                .get(&code)
                .unwrap_or_else(|| panic!("OperationCode {:x} is not recognized", code));

            match code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.load_accumulator(&operation_code.addressing_mode);
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.store_accumulator(&operation_code.addressing_mode);
                }
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.compare(&operation_code.addressing_mode, self.register_a);
                }
                0xE0 | 0xE4 | 0xEC => {
                    self.compare(&operation_code.addressing_mode, self.register_x);
                }
                0xC0 | 0xC4 | 0xCC => {
                    self.compare(&operation_code.addressing_mode, self.register_y);
                }
                0xAA => self.transfer_accumulator_to_x(),
                0xE8 => self.increment_x_register(),
                0x00 => return,
                _ => return,
            }

            if program_counter_previous == self.program_counter {
                self.program_counter += (operation_code.len - 1) as u16;
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.memory.read_u16(0xFFFC);
    }

    pub fn main(&mut self, program: Vec<u8>) {
        self.memory.load(program);
        self.reset();
        self.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a new CPU instance
    fn create_test_cpu() -> CPU {
        CPU {
            register_a: 0x05,
            register_x: 0x0A,
            register_y: 0x0F,
            status: 0x00,
            program_counter: 0x2000,
            memory: Memory::new(),
        }
    }

    #[test]
    fn test_immediate() {
        let mut cpu: CPU = create_test_cpu();

        let mode: AddressingMode = AddressingMode::Immediate;

        assert_eq!(cpu.get_operand_address(&mode), cpu.program_counter);
    }

    #[test]
    fn test_zero_page() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode: AddressingMode = AddressingMode::ZeroPage;

        assert_eq!(cpu.get_operand_address(&mode), data);
    }

    #[test]
    fn test_zero_page_x() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode: AddressingMode = AddressingMode::ZeroPage_X;

        assert_eq!(cpu.get_operand_address(&mode), data + cpu.register_x as u16);
    }

    #[test]
    fn test_zero_page_y() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0x80;
        cpu.memory.write_u16(cpu.program_counter, data);

        let mode = AddressingMode::ZeroPage_Y;
        assert_eq!(cpu.get_operand_address(&mode), data + cpu.register_y as u16);
    }

    #[test]
    fn test_get_operand_address_absolute() {
        let mut cpu: CPU = create_test_cpu();

        let data: u16 = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute;

        assert_eq!(cpu.get_operand_address(&mode), data);
    }

    #[test]
    fn test_get_operand_address_absolute_x() {
        let mut cpu: CPU = create_test_cpu();

        let data = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute_X;

        assert_eq!(cpu.get_operand_address(&mode), data + cpu.register_x as u16);
    }

    #[test]
    fn test_get_operand_address_absolute_y() {
        let mut cpu: CPU = create_test_cpu();

        let data = 0xC000;
        cpu.memory.write_u16(cpu.program_counter, data);
        let mode = AddressingMode::Absolute_Y;

        assert_eq!(cpu.get_operand_address(&mode), data + cpu.register_y as u16);
    }

    #[test]
    fn test_get_operand_address_indirect_x() {
        let mut cpu: CPU = create_test_cpu();

        let base: u8 = 0x20;
        let ptr: u8 = base.wrapping_add(cpu.register_x);
        cpu.memory.memory[cpu.program_counter as usize] = base;
        cpu.memory.memory[ptr as usize] = 0x34;
        cpu.memory.memory[ptr.wrapping_add(1) as usize] = 0x12;

        let mode = AddressingMode::Indirect_X;
        assert_eq!(cpu.get_operand_address(&mode), 0x1234);
    }

    #[test]
    fn test_get_operand_address_indirect_y() {
        let mut cpu: CPU = create_test_cpu();

        let base: u8 = 0x40;
        cpu.memory.memory[cpu.program_counter as usize] = base;
        cpu.memory.memory[base as usize] = 0x78;
        cpu.memory.memory[base.wrapping_add(1) as usize] = 0x56;

        let mode = AddressingMode::Indirect_Y;
        assert_eq!(
            cpu.get_operand_address(&mode),
            0x5678 + cpu.register_y as u16
        );
    }

    #[test]
    #[should_panic]
    fn test_get_operand_address_none_addressing() {
        let mut cpu: CPU = create_test_cpu();
        let mode = AddressingMode::NoneAddressing;
        cpu.get_operand_address(&mode);
    }
}
