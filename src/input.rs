use minifb::{Key, Window};

pub struct Input {
    pub keys: [bool; 16],
}

impl Input {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn update(&mut self, window: &Window) {
        let key_map = [
            Key::X,    // 0
            Key::Key1, // 1
            Key::Key2, // 2
            Key::Key3, // 3
            Key::Q,    // 4
            Key::W,    // 5
            Key::E,    // 6
            Key::A,    // 7
            Key::S,    // 8
            Key::D,    // 9
            Key::Z,    // A
            Key::C,    // B
            Key::Key4, // C
            Key::R,    // D
            Key::F,    // E
            Key::V,    // F
        ];

        for (chip8_key, &real_key) in key_map.iter().enumerate() {
            self.keys[chip8_key] = window.is_key_down(real_key);
        }
    }

    pub fn is_key_down(&self, chip8_key: usize) -> bool {
        self.keys[chip8_key]
    }
}
