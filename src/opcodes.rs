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
                    self.display.clear();
                }
            }
            0x1000 => {
                // 1NNN: goto addr NNN
                self.pc = nnn;
                return;
            }
            0x3000 => {
                // 3XNN: skip if VX == NN
                if self.v[x] == nn { self.pc += 2; }
            }
            0x4000 => {
                // 4XNN: skip if VX != NN
                if self.v[x] != nn { self.pc += 2; }
            }
            0x5000 => {
                // 5XY0: skip if VX == VY
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] { self.pc += 2; }
            }
            0x6000 => {
                // 6XNN: set reg VX to NN
                self.v[x] = nn;
            }
            0x7000 => {
                // 7XNN: add NN to reg VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x9000 => {
                // 9XY0: skip if VX != VY
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] { self.pc += 2; }
            }
            0xA000 => {
                // ANNN: set idx reg I to addr NNN
                self.i = nnn;
            }
            0xD000 => {
                // DXYN: draw an N tall sprite from addr I at (VX, VY)
                let n = (opcode & 0x000F) as usize;
                let vx = self.v[x] as usize;
                let vy = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;

                let mut sprite_data = Vec::new();
                for row in 0..n {
                    sprite_data.push(self.memory.read(self.i as usize + row));
                }

                let collision = self.display.draw_sprite(vx, vy, &sprite_data);
                self.v[0xF] = collision as u8;
            }
            _ => {
                // not implemented yet
                println!("Unimplemented opcode: {opcode:#06X}");
            }
        }
        self.pc += 2
    }
}
