use std::{fs};

struct ProgramCounter {
    count: u16
}

// NOTE: Stored with Most-Significant-Bit first (left to right)
// 1. the first byte should be located at a even address
// 2. if a program has sprites, I should pad something so
// any instructions following will be properly put in the ram

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
    let as_u16 = convert_u8_to_u16(&blitz);
    let asas = as_u16.expect("didnt work :(");
    u16_to_decimal(asas);
    // let chip8 = Chip8::new();
    // chip8.interpret(blitz);
}

fn load_rom(filename: &str) -> Vec<u8> {
    fs::read(filename).expect("Could not load ROM")
}
