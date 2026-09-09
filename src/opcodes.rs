use crate::cpu::Cpu;

impl Cpu {
    pub fn execute(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => {
                if opcode == 0x00E0 {
                    // 00E0: clear screen
                    println!("Clear screen (not implemented yet)");
                }
            }
            0x1000 => {
                // 1NNN: goto addr NNN
                self.pc = nnn;
                return;
            }
            0x6000 => {
                // 6XNN: set reg VX to NN
                self.v[x] = nn;
            }
            0x7000 => {
                // 7XNN: add NN to reg VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0xA000 => {
                // ANNN: set idx reg I to addr NNN
                self.i = nnn;
            }
            _ => {
                // not implemented yet
                println!("Unimplemented opcode: {opcode:#06X}");
            }
        }
        self.pc += 2
    }
}
