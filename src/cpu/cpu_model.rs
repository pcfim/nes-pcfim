use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::memory::Memory;
pub type ExecuteFunction = fn(&mut CPU, &AddressingMode);
pub const STACK: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xfd;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub memory: Memory,
}
