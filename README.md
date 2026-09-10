# Chip-8 Emulator

A Chip-8 emulator written in Rust, built as my learning project to learn how CPU emulation, memory management, and bitwise operations work.

## Features

- Full Chip-8 instruction set (all ~35 opcodes)
- 64x32 monochrome display via `minifb`
- 16-key hex keypad input, mapped to a standard keyboard layout
- Accurate 60Hz delay/sound timers, decoupled from CPU speed
- Unit + integration tested (`cargo test`)
- CI via GitHub Actions - tests run automatically on every push/PR

## Running it

```bash
cargo run
```

Place a Chip-8 ROM file in the `roms/` folder and update the filename in `src/main.rs` to point at it (or keep `IBM_Logo.ch8` to see the builtin test image).

## Controls

Chip-8 was designed for a 16-key hex keypad. This emulator maps it to your keyboard like this:

| Chip-8 keypad                            | Your keyboard                            |
| ---------------------------------------- | ---------------------------------------- |
| 1 2 3 C<br>4 5 6 D<br>7 8 9 E<br>A 0 B F | 1 2 3 4<br>Q W E R<br>A S D F<br>Z X C V |

Press `Esc` to quit.

## Tests

```bash
cargo test
```

## A note on ROMs

`roms/*.ch8` and `roms/*.rom` are gitignored by default, since Chip-8 game ROMs are often copyrighted even decades later. The IBM logo test ROM and public domain test suites are force added exceptions - see `.gitignore` for details. If you add your own ROMs to test with, keep in mind they wont be tracked unless you explicitly force add them.
