use std::{env, fs, path::PathBuf, str::from_utf8, vec};

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
    // read_byte_by_byte_from_file(&blitz);
    // read_rom_as_hex(&blitz);
    // decimal_to_binary_only(&blitz);
    identify_cls_and_ret(&blitz);
}

fn load_rom(filename: &str) -> Vec<u8> {
    let mut path = PathBuf::from("roms/chip8-roms/games");
    path.push(filename);

    fs::read(&path)
        .unwrap_or_else(|err| panic!("Could not load ROM: {}, reason: {}", path.display(), err))
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
