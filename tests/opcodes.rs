use chip8::cpu::Cpu;

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
fn opcode_8xy0_copies_register() {
    let mut cpu = Cpu::new();
    cpu.v[2] = 0x77;
    cpu.execute(0x8120);
    assert_eq!(cpu.v[1], 0x77);
}

#[test]
fn opcode_8xy4_adds_with_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0xFF;
    cpu.v[1] = 0x02;
    cpu.execute(0x8014);
    assert_eq!(cpu.v[0], 0x01);
    assert_eq!(cpu.v[0xF], 1);
}

#[test]
fn opcode_8xy4_adds_without_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x10;
    cpu.v[1] = 0x05;
    cpu.execute(0x8014);
    assert_eq!(cpu.v[0], 0x15);
    assert_eq!(cpu.v[0xF], 0);
}

#[test]
fn opcode_8xy5_subtracts_no_borrow() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x10;
    cpu.v[1] = 0x05;
    cpu.execute(0x8015);
    assert_eq!(cpu.v[0], 0x0B);
    assert_eq!(cpu.v[0xF], 1);
}

#[test]
fn opcode_8xy5_subtracts_with_borrow() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x05;
    cpu.v[1] = 0x10;
    cpu.execute(0x8015);
    assert_eq!(cpu.v[0xF], 0);
}

#[test]
fn opcode_8xy6_shifts_right_and_captures_bit() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0b0000_0011;
    cpu.execute(0x8006);
    assert_eq!(cpu.v[0], 0b0000_0001);
    assert_eq!(cpu.v[0xF], 1);
}

#[test]
fn opcode_8xy7_subtracts_reversed_order() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x05;
    cpu.v[1] = 0x10;
    cpu.execute(0x8017);
    assert_eq!(cpu.v[0], 0x0B);
    assert_eq!(cpu.v[0xF], 1);
}

#[test]
fn opcode_8xy7_subtracts_reversed_order_with_borrow() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x10;
    cpu.v[1] = 0x05;
    cpu.execute(0x8017);
    assert_eq!(cpu.v[0xF], 0);
}

#[test]
fn opcode_8xye_shifts_left_and_captures_bit() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0b1000_0001;
    cpu.execute(0x800E);
    assert_eq!(cpu.v[0], 0b0000_0010);
    assert_eq!(cpu.v[0xF], 1);
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

#[test]
fn opcode_annn_sets_index_register() {
    let mut cpu = Cpu::new();
    cpu.execute(0xA123);
    assert_eq!(cpu.i, 0x123);
}
