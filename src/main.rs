use std::{env, fs, path::PathBuf, str::from_utf8, vec};

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
    read_byte_by_byte_from_file(&blitz);
    read_rom_as_hex(&blitz);
}

fn load_rom(filename: &str) -> Vec<u8> {
    let mut path = PathBuf::from("roms/chip8-roms/games");
    path.push(filename);

    fs::read(&path)
        .unwrap_or_else(|err| panic!("Could not load ROM: {}, reason: {}", path.display(), err))
}

fn read_byte_by_byte_from_file(bytes: &Vec<u8>) {
    for byte in bytes {
        println!("{}", byte);
    }
}

fn read_rom_as_hex(rom: &Vec<u8>) {
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 { println!(""); }
    }

    println!("");
}
