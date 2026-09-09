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

#[test]
fn opcode_3xnn_skips_when_equal() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x42;
    let pc_before = cpu.pc;
    cpu.execute(0x3042);
    assert_eq!(cpu.pc, pc_before + 4);
}

#[test]
fn opcode_3xnn_does_not_skip_when_not_equal() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x12;
    let pc_before = cpu.pc;
    cpu.execute(0x3042);
    assert_eq!(cpu.pc, pc_before + 2);
}

#[test]
fn opcode_4xnn_skips_when_not_equal() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x12;
    let pc_before = cpu.pc;
    cpu.execute(0x4042);
    assert_eq!(cpu.pc, pc_before + 4);
}

#[test]
fn opcode_5xy0_skips_when_equal() {
    let mut cpu = Cpu::new();
    cpu.v[1] = 0x20;
    cpu.v[2] = 0x20;
    let pc_before = cpu.pc;
    cpu.execute(0x5120);
    assert_eq!(cpu.pc, pc_before + 4);
}

#[test]
fn opcode_9xy0_skips_when_not_equal() {
    let mut cpu = Cpu::new();
    cpu.v[1] = 0x20;
    cpu.v[2] = 0x22;
    let pc_before = cpu.pc;
    cpu.execute(0x9120);
    assert_eq!(cpu.pc, pc_before + 4);
}
