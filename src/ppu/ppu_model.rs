use super::super::cartridge::mirroring::Mirroring;
use super::{address_register::AddressRegister, control_register::ControlRegister};

pub struct NesPPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub address: AddressRegister,
    pub control_register: ControlRegister,
    pub internal_data_buf: u8,
}
