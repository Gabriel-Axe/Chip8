use std::{env, fs, path::PathBuf, str::from_utf8, vec};

struct GeneralRegister {
    data: Vec<u8>
}

impl GeneralRegister {
    
    fn new() -> GeneralRegister {
        return GeneralRegister{
            data: Vec::new()
        };
    }
}

struct IRegister {
    data: Vec<u16>
}

struct DelayTimerRegister {
    current: u8
}

struct SoundTimerRegister {
    current: u8
}

struct ProgramCounter {
    count: u16
}

// NOTE: Never heard of this...
struct StackPointer {
    address: u8
}

type Bit16 = [bool; 16];
type Stack = [Bit16; 16];

fn memory() {
    let r1 = GeneralRegister::new();
    let r2 = GeneralRegister::new();
    let r3 = GeneralRegister::new();
    let r4 = GeneralRegister::new();
    let r5 = GeneralRegister::new();
    let r6 = GeneralRegister::new();
    let r7 = GeneralRegister::new();
    let r8 = GeneralRegister::new();
    let r9 = GeneralRegister::new();
    let r10 = GeneralRegister::new();
    let r11 = GeneralRegister::new();
    let r12 = GeneralRegister::new();
    let r13 = GeneralRegister::new();
    let r14 = GeneralRegister::new();
    let r15 = GeneralRegister::new();
    let r16 = GeneralRegister::new();

    let regI = IRegister{data: Vec::new()};
    let reg_delay = DelayTimerRegister{current: 0};
    let reg_sound = SoundTimerRegister{current: 0};
    let pc = ProgramCounter{count: 0};
    let sp = StackPointer{address: 0};
}

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
