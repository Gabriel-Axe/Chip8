use crate::chip8::Chip8;

mod chip8;
mod cpu;
mod memory;
mod register;
mod util;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.run();
}
