use std::{fs};

use crate::util::{convert_u8_to_u16, get_instruction_nibble, print_bytes_in_16_binary, print_bytes_in_binary};

mod util;
mod chip8;
mod memory;

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    for instruction in blitz {
        let inst_type = get_instruction_nibble(instruction);
        match  {
            
        }
    }
}

fn load_rom(filename: &str) -> Vec<u16> {
    let file = fs::read(filename).expect("Could not load ROM");
    let file_as_u16 = convert_u8_to_u16(&file).expect("Could not convert ROM to u16");
    file_as_u16
}
