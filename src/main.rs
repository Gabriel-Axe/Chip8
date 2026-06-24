use std::{env, fs, path::PathBuf};

fn main() {
    let blitz = load_rom("Blitz [David Winter].ch8");
    println!("Loaded Blitz: {} bytes", blitz.len());
}

fn load_rom(filename: &str) -> Vec<u8> {
    let mut path = PathBuf::from("roms/chip8-roms/games");
    path.push(filename);

    fs::read(&path)
        .unwrap_or_else(|err| panic!("Could not load ROM: {}, reason: {}", path.display(), err))
}
