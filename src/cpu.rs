use crate::display::Display;
use crate::memory::{Memory, PROGRAM_START};

#[allow(dead_code)]
pub struct Cpu {
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: Memory,
    pub display: Display,
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START as u16,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            memory: Memory::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory.load_rom(rom_data);
    }

    pub fn cycle(&mut self) {
        let high_byte = self.memory.read(self.pc as usize);
        let low_byte = self.memory.read(self.pc as usize + 1);
        let opcode = ((high_byte as u16) << 8) | (low_byte as u16);

        self.execute(opcode);
    }
}
