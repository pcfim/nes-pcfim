use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::memory::Memory;
pub type ExecuteFunction = fn(&mut CPU, &AddressingMode);
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: Memory,
}
