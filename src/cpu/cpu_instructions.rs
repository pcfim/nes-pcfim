use super::addressing_mode::AddressingMode;
use super::operation_codes;
use crate::bus::bus_handler::Bus;
use crate::cpu::cpu_model::ExecuteFunction;
use crate::cpu::cpu_model::CPU;
use crate::cpu::cpu_model::STACK_RESET;
use std::collections::HashMap;

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos.wrapping_add(1)) as u16;
        (hi << 8) | (lo)
    }
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos.wrapping_add(1), hi);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b100100,
            stack_pointer: STACK_RESET,
            program_counter: 0,
            bus: Bus::new(),
        }
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }
    pub fn run_once(&mut self) {
        let operation_codes: &HashMap<u8, (&'static operation_codes::Operation, ExecuteFunction)> =
            &operation_codes::OPERATION_CODES_MAP;
        let code = self.bus.mem_read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        let program_counter_previous = self.program_counter;
        let (operation_code, execute_function) = operation_codes
            .get(&code)
            .unwrap_or_else(|| panic!("OperationCode {:x} is not recognized", code));

        if operation_code.operation_code == 0x00 {
            return;
        }
        execute_function(self, &operation_code.addressing_mode);

        if (operation_code.addressing_mode == AddressingMode::Relative
            && self.program_counter != program_counter_previous
            && self.bus.mem_read(program_counter_previous) == 255)
            || program_counter_previous == self.program_counter
        {
            self.program_counter = self
                .program_counter
                .wrapping_add(operation_code.len as u16 - 1);
        }
    }
    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        let operation_codes: &HashMap<u8, (&'static operation_codes::Operation, ExecuteFunction)> =
            &operation_codes::OPERATION_CODES_MAP;
        loop {
            let code = self.bus.mem_read(self.program_counter);
            self.program_counter = self.program_counter.wrapping_add(1);
            if code == 0xEA {
                continue;
            }
            let program_counter_previous = self.program_counter;
            let (operation_code, execute_function) = operation_codes
                .get(&code)
                .unwrap_or_else(|| panic!("OperationCode {:x} is not recognized", code));

            if operation_code.operation_code == 0x00 {
                return;
            }
            execute_function(self, &operation_code.addressing_mode);

            if (operation_code.addressing_mode == AddressingMode::Relative
                && self.program_counter != program_counter_previous
                && self.bus.mem_read(program_counter_previous) == 255)
                || program_counter_previous == self.program_counter
            {
                self.program_counter = self
                    .program_counter
                    .wrapping_add(operation_code.len as u16 - 1);
            }
            callback(self);
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0b00100100;
        self.stack_pointer = STACK_RESET;
        self.program_counter = self.bus.mem_read_u16(0xFFFC);
    }

    pub fn main(&mut self, program: Vec<u8>) {
        self.bus.load(program);
        self.reset();
        self.run();
    }
}
#[test]
fn test_branch_operations() {
    let mut cpu = CPU::new();
    let program = vec![
        0x18, 0x90, 0x04, 0xa9, 0x00, 0x85, 0x00, 0xa9, 0x01, 0x85, 0x00, 0x38, 0xb0, 0x04, 0xa9,
        0x00, 0x85, 0x01, 0xa9, 0x01, 0x85, 0x01, 0xa9, 0x05, 0xc9, 0x05, 0xf0, 0x04, 0xa9, 0x00,
        0x85, 0x02, 0xa9, 0x01, 0x85, 0x02, 0xc9, 0x06, 0xd0, 0x04, 0xa9, 0x00, 0x85, 0x03, 0xa9,
        0x01, 0x85, 0x03, 0xa9, 0x80, 0x30, 0x04, 0xa9, 0x00, 0x85, 0x04, 0xa9, 0x01, 0x85, 0x04,
        0xa9, 0x7f, 0x10, 0x04, 0xa9, 0x00, 0x85, 0x05, 0xa9, 0x01, 0x85, 0x05, 0x18, 0xa9, 0x7f,
        0x69, 0x01, 0x70, 0x04, 0xa9, 0x00, 0x85, 0x06, 0xa9, 0x01, 0x85, 0x06, 0x18, 0xa9, 0x40,
        0x69, 0x20, 0x50, 0x04, 0xa9, 0x00, 0x85, 0x07, 0xa9, 0x01, 0x85, 0x07, 0x00,
    ];

    cpu.main(program);

    // Assertions
    assert_eq!(
        cpu.bus.mem_read(0x00),
        0x01,
        "BCC failed: Expected branch when Carry clear"
    );
    assert_eq!(
        cpu.bus.mem_read(0x01),
        0x01,
        "BCS failed: Expected branch when Carry set"
    );
    assert_eq!(
        cpu.bus.mem_read(0x02),
        0x01,
        "BEQ failed: Expected branch when equal"
    );
    assert_eq!(
        cpu.bus.mem_read(0x03),
        0x01,
        "BNE failed: Expected branch when not equal"
    );
    assert_eq!(
        cpu.bus.mem_read(0x04),
        0x01,
        "BMI failed: Expected branch when negative"
    );
    assert_eq!(
        cpu.bus.mem_read(0x05),
        0x01,
        "BPL failed: Expected branch when positive"
    );
    assert_eq!(
        cpu.bus.mem_read(0x06),
        0x01,
        "BVS failed: Expected branch when Overflow set"
    );
    assert_eq!(
        cpu.bus.mem_read(0x07),
        0x01,
        "BVC failed: Expected branch when Overflow clear"
    );
}
