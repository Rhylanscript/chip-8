use chip8::cpu::Cpu;

#[test]
fn opcode_6xnn_sets_register() {
    let mut cpu = Cpu::new();
    cpu.execute(0x6A15); // sets V[A] to 0x15
    assert_eq!(cpu.v[0xA], 0x15);
}

#[test]
fn opcode_7xnn_adds_to_register() {
    let mut cpu = Cpu::new();
    cpu.v[3] = 0x10;
    cpu.execute(0x7305); // adds 0x05 to V[3]
    assert_eq!(cpu.v[3], 0x15);
}

#[test]
fn opcode_annn_sets_index_register() {
    let mut cpu = Cpu::new();
    cpu.execute(0xA123);
    assert_eq!(cpu.i, 0x123);
}

#[test]
fn opcode_1nnn_jumps_pc() {
    let mut cpu = Cpu::new();
    cpu.execute(0x1300);
    assert_eq!(cpu.pc, 0x300);
}
