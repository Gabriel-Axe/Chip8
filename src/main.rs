use std::{fs};

use crate::util::{convert_u8_to_u16, u16_to_decimal};

mod util;

struct ProgramCounter {
    count: u16
}

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
    let as_u16 = convert_u8_to_u16(&blitz);
    let asas = as_u16.expect("didnt work :(");
    u16_to_decimal(asas);
}

fn load_rom(filename: &str) -> Vec<u8> {
    fs::read(filename).expect("Could not load ROM")
}
