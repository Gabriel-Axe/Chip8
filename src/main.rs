use std::{fs, path::PathBuf};

use crate::{chip8::Chip8, disassembler::Disassembler};

mod disassembler;
mod chip8;
mod cpu;
mod display;
mod memory;
mod debug_printer;
mod register;
mod util;

fn main() {
    let rom = load_rom("/home/gabriel/Programing/Projects/Rust/Chip8/test_suite/bin", "2-ibm-logo.ch8");
    env_logger::init();
    let mut chip8 = Chip8::new();
    chip8.load_rom("IBM Logo.ch8");
    chip8.run();
    // Disassembler::output_to_file(rom);
}

pub fn load_rom(path: &str, filename: &str) -> Vec<u8> {
    let mut path = PathBuf::from(path);
    path.push(filename);

    let file  = fs::read(&path)
        .unwrap_or_else(|err| {
            panic!("Could not load ROM: {}, reason: {}", path.display(), err)
        });
    file
}
