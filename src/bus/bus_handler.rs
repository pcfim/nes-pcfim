use crate::cpu::cpu_instructions::Mem;
// const RAM: u16 = 0x0000;
// const RAM_PARSER: u16 = 0x07FF;
// const RAM_MIRRORS_END: u16 = 0x1FFF;

// const PPU_REGISTERS: u16 = 0x2000;
// const PPU_PARSER: u16 = 0x2007;
// const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
#[derive(PartialEq, Eq, Debug)]
pub struct Bus {
    cpu_vram: [u8; 65536],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; 65536],
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let start_position: usize = 0x0600;
        self.cpu_vram[start_position..(start_position + program.len())]
            .copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, start_position as u16);
    }
}
impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
impl Mem for Bus {
    fn mem_read(&self, address: u16) -> u8 {
        let position = address;
        self.cpu_vram[position as usize]

        // match address {
        // 	RAM..=RAM_MIRRORS_END => {
        // 		let position = address & RAM_PARSER;
        // 		self.cpu_vram[position as usize]
        // 	},
        // 	PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
        // 		let _position = address & PPU_PARSER;
        // 		todo!("PPU is not supported yet");
        // 	}
        // 	_ => {
        // 		println!("{} {:04X}", address, address);
        // 		println!("Ignoring bad memory access");
        // 		0
        // 	}
        // }
    }
    fn mem_write(&mut self, address: u16, data: u8) {
        let position = address;
        self.cpu_vram[position as usize] = data;
        // match address {
        // 	RAM..=RAM_MIRRORS_END => {
        // 		let position = address & RAM_PARSER;
        // 		self.cpu_vram[position as usize] = data;
        // 	},
        // 	PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
        // 		let _position = address & PPU_PARSER;
        // 		todo!("PPU is not supported yet");
        // 	}
        // 	_ => {}
        // }
    }
}
