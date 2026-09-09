use crate::{cpu::Cpu, memory::FONT_START};

impl Cpu {
    pub fn execute(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => {
                if opcode == 0x00E0 {
                    // 00E0: clear screen
                    self.display.clear();
                } else if opcode == 0x00EE {
                    // 00EE: return from subroutine
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    return;
                }
            }
            0x1000 => {
                // 1NNN: goto addr NNN
                self.pc = nnn;
                return;
            }
            0x2000 => {
                // 2NNN: call subroutine at addr NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
                return;
            }
            0x3000 => {
                // 3XNN: skip if VX == NN
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 4XNN: skip if VX != NN
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // 5XY0: skip if VX == VY
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 6XNN: set reg VX to NN
                self.v[x] = nn;
            }
            0x7000 => {
                // 7XNN: add NN to reg VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x8000 => match opcode & 0x000F {
                0x0 => {
                    // 8XY0: VX = VY (just simple copy)
                    self.v[x] = self.v[y];
                }
                0x1 => {
                    // 8XY1: VX = VX OR VY
                    self.v[x] |= self.v[y];
                }
                0x2 => {
                    // 8XY2: VX = VX AND VY
                    self.v[x] &= self.v[y];
                }
                0x3 => {
                    // 8XY3: VX = VX XOR VY
                    self.v[x] ^= self.v[y];
                }
                0x4 => {
                    // 8XY4: VX += VY, VF = 1 if overflowed past 255 else 0
                    let (result, overflowed) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = overflowed as u8;
                }
                0x5 => {
                    // 8XY5: VX -= VY, VF = 1 if NO borrow occurred (VX >= VY) else 0
                    let (result, borrowed) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = !borrowed as u8; // inverted
                }
                0x6 => {
                    // 8XY6: shift VX right by 1 | VF = bit that got shifted out
                    let shifted_out_bit = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                    self.v[0xF] = shifted_out_bit;
                }
                0x7 => {
                    // 8XY7: VX = VY - VX same VF rule
                    let (result, borrowed) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = result;
                    self.v[0xF] = !borrowed as u8;
                }
                0xE => {
                    // 8XYE: shift VX left by 1 | same VF rule as 8XY6
                    let shifted_out_bit = (self.v[x] & 0x80) >> 7;
                    self.v[x] <<= 1;
                    self.v[0xF] = shifted_out_bit;
                }
                _ => {
                    println!("Unimplemented 0x8000 family opcode: {opcode:#06X}");
                }
            },
            0x9000 => {
                // 9XY0: skip if VX != VY
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
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
            0xF000 => match opcode & 0x00FF {
                0x07 => {
                    // FX07: VX = current delay timer value
                    self.v[x] = self.delay_timer;
                }
                0x15 => {
                    // FX15: delay timer = VX
                    self.delay_timer = self.v[x];
                }
                0x18 => {
                    // FX18: sound timer = VX
                    self.sound_timer = self.v[x];
                }
                0x1E => {
                    // FX1E: I += VX
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                0x29 => {
                    // FX29: I = addr of font sprite for digit VX
                    // FONT_START + (N * 5)
                    self.i = (FONT_START + (self.v[x] as usize * 5)) as u16;
                }
                0x33 => {
                    // FX33: split VX into hundreds/tens/ones
                    let value = self.v[x];
                    let hundreds = value / 100;
                    let tens = (value / 10) % 10;
                    let ones = value % 10;

                    self.memory.write(self.i as usize, hundreds);
                    self.memory.write(self.i as usize + 1, tens);
                    self.memory.write(self.i as usize + 2, ones);
                }
                0x55 => {
                    // FX55: store V0..=VX into memory starting at I
                    for offset in 0..=x {
                        self.memory.write(self.i as usize + offset, self.v[offset]);
                    }
                }
                0x65 => {
                    // FX65: load memory starting at I into V0..=VX
                    for offset in 0..=x {
                        self.v[offset] = self.memory.read(self.i as usize + offset);
                    }
                }
                _ => {
                    println!("Unimplemented 0xF000-family opcode: {opcode:#06X}");
                }
            },
            _ => {
                // not implemented yet
                println!("Unimplemented opcode: {opcode:#06X}");
            }
        }
        self.pc += 2
    }
}
