use std::fs;

use chip8::cpu::Cpu;
use chip8::display::{HEIGHT, WIDTH};
use minifb::{Key, Window, WindowOptions};

fn main() {
    let rom_data = fs::read("roms/IBM_Logo.ch8").expect("failed to read ROM file");

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_data);

    let mut window = Window::new(
        "Chip-8 Emulator",
        WIDTH * 10,
        HEIGHT * 10,
        WindowOptions::default(),
    )
    .expect("failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Run a handful of CPU cycles per frame. This is a rough
        // approximation for now — we'll tune real timing in Milestone 5.
        for _ in 0..10 {
            cpu.cycle();
        }

        // Convert our bool pixel array into the u32 color format
        // minifb expects, scaling each Chip-8 pixel up into a 10x10
        // block of actual window pixels.
        let mut buffer = vec![0u32; WIDTH * 10 * HEIGHT * 10];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if cpu.display.pixels[y * WIDTH + x] {
                    0x00FFFFFF // white
                } else {
                    0x00000000 // black
                };
                for dy in 0..10 {
                    for dx in 0..10 {
                        let real_x = x * 10 + dx;
                        let real_y = y * 10 + dy;
                        buffer[real_y * (WIDTH * 10) + real_x] = color;
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH * 10, HEIGHT * 10)
            .expect("failed to update window");
    }
}
