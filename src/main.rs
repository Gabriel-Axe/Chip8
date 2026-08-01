use core::num;
use std::{env, fs, io::Bytes, path::PathBuf, str::{FromStr, from_utf8}, vec};


struct ProgramCounter {
    count: u16
}

// NOTE: Never heard of this...
struct StackPointer {
    address: u8
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

fn get_4th_nibble(instruction: Bit16) {
    let 1st_nibble = instruction
    let 2th_nibble = ;
    let 3th_nibble = ;
    let 4th_nibble = ;
}

fn load_rom(filename: &str) -> Vec<u8> {
    fs::read(filename).expect("Could not load ROM")
}

// fn convert_8bits_to_16bits(bytes: &Vec<u8>) -> Vec<u16> {
fn convert_u8_to_u16(bytes: &[u8]) -> Result<Vec<u16>, String> {
    // if bytes.len() % 2 != 0 {
    //     let res = String::from_str("Cannot convert u8 vector, there is a odd number of values");
    //     let mes = res.expect("Could not unwrap :(");
    //     return Err(mes);
    // }

    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    return Ok(words)
}

fn identify_cls_and_ret(bytes: &Vec<u8>) {
    for byte in bytes  {
        match byte {
            224 => { print!("Its a cls: "); decimal_to_binary_one_number(byte);}
            238 => { print!("Its a ret: "); decimal_to_binary_one_number(byte); }
            252 => { print!("Its a scr: "); decimal_to_binary_one_number(byte); }
            253 => { print!("Its a exit: "); decimal_to_binary_one_number(byte); }
            254 => { print!("Its a low: "); decimal_to_binary_one_number(byte); }
            255 => { print!("Its a high: "); decimal_to_binary_one_number(byte); }
            // _ => { decimal_to_binary_one_number(byte);}
            _ => {}
        }
    }
}

fn read_byte_by_byte_from_file(bytes: &Vec<u8>) {
    for byte in bytes {
        println!("{}", byte);
    }
}

fn decimal_to_binary_one_number(number: &u8) {
    println!("{:08b}", number);
}

fn u16_to_decimal(numbers: Vec<u16>) {
    for num in numbers {
        println!("{:016b}", num);
    }
}

fn decimal_to_binary_only(numbers: &[u8]) {
    for &num in numbers {
        println!("{:08b}", num);
    }
}

fn read_rom_as_hex(rom: &Vec<u8>) {
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 { println!(""); }
    }

    println!("");
}
