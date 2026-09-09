use std::fs;

use chip8::cpu::Cpu;

fn main() {
    let rom_data = fs::read("roms/IBM_Logo.ch8").expect("failed to read ROM file");

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_data);

    for _ in 0..20 {
        cpu.cycle();
    }
}
