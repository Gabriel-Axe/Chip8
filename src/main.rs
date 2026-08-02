use std::{fs};

use crate::{chip8::Chip8, util::{convert_bytes_u8_to_u16, get_instruction_nibble, print_bytes_in_16_binary, print_bytes_in_binary}};

mod util;
mod chip8;
mod memory;

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    let chip8 = Chip8::new();
    chip8.interpret(blitz);
}

fn load_rom(filename: &str) -> Vec<u16> {
    let file = fs::read(filename).expect("Could not load ROM");
    let file_as_u16 = convert_bytes_u8_to_u16(&file).expect("Could not convert ROM to u16");
    file_as_u16
}
