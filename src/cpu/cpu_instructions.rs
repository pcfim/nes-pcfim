use super::operation_codes;
use crate::cpu::cpu_model::ExecuteFunction;
use crate::cpu::cpu_model::CPU;
use crate::cpu::cpu_model::STACK_RESET;
use crate::cpu::memory::Memory;
use std::collections::HashMap;

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
            stack_pointer: STACK_RESET,
            program_counter: 0,
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        let operation_codes: &HashMap<u8, (&'static operation_codes::Operation, ExecuteFunction)> =
            &operation_codes::OPERATION_CODES_MAP;
        loop {
            let code = self.memory.memory[self.program_counter as usize];
            self.program_counter += 1;
            let program_counter_previous = self.program_counter;
            let (operation_code, execute_function) = operation_codes
                .get(&code)
                .unwrap_or_else(|| panic!("OperationCode {:x} is not recognized", code));
            execute_function(self, &operation_code.addressing_mode);

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
