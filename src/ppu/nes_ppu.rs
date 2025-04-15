use super::super::cartridge::mirroring::Mirroring;
use super::address_constants::{
    CHR_ADDRESS_HIGH, CHR_ADDRESS_LOW, PPU_ADDRESS_HIGH, PPU_ADDRESS_LOW, RAM_ADDRESS_HIGH,
    RAM_ADDRESS_LOW, UNUSED_ADDRESS_HIGH, UNUSED_ADDRESS_LOW,
};
use super::{
    address_register::AddressRegister, control_register::ControlRegister, ppu_model::NesPPU,
};

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        NesPPU {
            chr_rom,
            mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            address: AddressRegister::new(),
            control_register: ControlRegister::new(),
            internal_data_buf: 0,
        }
    }
    #[allow(dead_code)]
    fn write_to_ppu_address(&mut self, value: u8) {
        self.address.update(value);
    }

    #[allow(dead_code)]
    fn write_to_control_register(&mut self, value: u8) {
        self.control_register.update(value);
    }

    pub fn mirror_vram_address(&self, address: u16) -> u16 {
        let mirrored_vram = address & 0b10111111111111;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    #[allow(dead_code)]
    fn increment_vram_address(&mut self) {
        self.address
            .increment(self.control_register.vram_address_increment());
    }

    #[allow(dead_code)]
    fn read_data(&mut self) -> u8 {
        let address: u16 = self.address.get();
        self.increment_vram_address();

        match address {
            CHR_ADDRESS_LOW..=CHR_ADDRESS_HIGH => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[address as usize];
                result
            }
            RAM_ADDRESS_LOW..=RAM_ADDRESS_HIGH => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_address(address) as usize];
                result
            }
            UNUSED_ADDRESS_LOW..=UNUSED_ADDRESS_HIGH => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                address
            ),
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = address - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }
            PPU_ADDRESS_LOW..=PPU_ADDRESS_HIGH => {
                self.palette_table[(address - PPU_ADDRESS_LOW) as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", address),
        }
    }
}
