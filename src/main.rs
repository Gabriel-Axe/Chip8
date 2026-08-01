use std::{fs};

use crate::util::{convert_u8_to_u16, get_4th_nibble, print_bytes_in_16_binary, print_bytes_in_binary};

mod util;

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
    let as_u16 = convert_u8_to_u16(&blitz);
    let blitz16 = as_u16.expect("Coudn't convert blitz into u16");
    // print_bytes_in_16_binary(&blitz16);
    let blitz_4ths = get_4th_nibble(&blitz16);
    print_bytes_in_16_binary(&blitz_4ths);
}

fn load_rom(filename: &str) -> Vec<u8> {
    fs::read(filename).expect("Could not load ROM")
}
