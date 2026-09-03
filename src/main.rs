use crate::chip8::Chip8;

mod chip8;
mod cpu;
mod display;
mod memory;
mod chip8_tests;
mod debug_printer;
mod register;
mod util;

fn main() {
    env_logger::init();
    let mut chip8 = Chip8::new();
    // chip8.load_rom("IBM Logo.ch8");
    // chip8.load_rom("1-chip8-logo.ch8");
    chip8.load_rom("3-corax+.ch8");
    chip8.run();
}
