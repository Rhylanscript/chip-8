use std::fs;

use chip8::cpu::Cpu;

mod memory;
mod cpu;

fn main() {
    let rom_data = fs::read("roms/IBM_Logo.ch8").expect("failed to read ROM file");

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_data);

    println!("Loaded {} bytes into memory at 0x200", rom_data.len());
}
