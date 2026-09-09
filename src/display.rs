pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub pixels: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [false; WIDTH * HEIGHT];
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite_data: &[u8]) -> bool {
        let mut collision = false;

        for (row, byte) in sprite_data.iter().enumerate() {
            for bit in 0..8 {
                let sprite_pixel_on = (byte << bit) & 0x80 != 0;

                if !sprite_pixel_on {
                    continue;
                }

                let screen_x = (x + bit) % WIDTH;
                let screen_y = (y + row) % HEIGHT;
                let index = screen_y * WIDTH + screen_x;

                if self.pixels[index] {
                    collision = true;
                }
                self.pixels[index] ^= true;
            }
        }

        collision
    }
}
