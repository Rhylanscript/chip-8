use std::fs;

use chip8::cpu::Cpu;

#[test]
fn rom_loads_correctly_at_0x200() {
    let rom_data = fs::read("roms/IBM_Logo.ch8").expect("failed to read ROM file");

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_data);

    for (offset, &expected_byte) in rom_data.iter().enumerate() {
        let actual_byte = cpu.memory.read(0x200 + offset);

        assert_eq!(
            actual_byte, expected_byte,
            "ROM mismatch at offset {offset}"
        )
    }
}