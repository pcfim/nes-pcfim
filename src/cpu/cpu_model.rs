use crate::bus::bus_handler::Bus;
use crate::cpu::addressing_mode::AddressingMode;
pub type ExecuteFunction = fn(&mut CPU, &AddressingMode);
pub const STACK: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xfd;
#[derive(PartialEq, Eq, Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub bus: Bus,
}
