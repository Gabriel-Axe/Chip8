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

// NOTE: Intended for instructions with x and y
type Bit4 = [bool; 4];

// NOTE: Aka Byte
// Usage at kk
type Bit8 = [bool; 8];

type Bit16 = [bool; 16];
type Stack = [Bit16; 16];

// NOTE: Usage at n
type Nibble = (bool, bool, bool, bool);

// NOTE: Usage at nnn
type Address = (
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool);

// NOTE: Stored with Most-Significant-Bit first (left to right)
// 1. the first byte should be located at a even address
// 2. if a program has sprites, I should pad something so
// any instructions following will be properly put in the ram
type Instruction = (bool, bool);

struct Memory {
    r1: GeneralRegister,
    r2: GeneralRegister,
    r3: GeneralRegister,
    r4: GeneralRegister,
    r5: GeneralRegister,
    r6: GeneralRegister,
    r7: GeneralRegister,
    r8: GeneralRegister,
    r9: GeneralRegister,
    r10: GeneralRegister,
    r11: GeneralRegister,
    r12: GeneralRegister,
    r13: GeneralRegister,
    r14: GeneralRegister,
    r15: GeneralRegister,
    r16: GeneralRegister,
    regI: IRegister,
}

impl Memory {
    fn new() -> Self {
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
        Memory {
            r1: r1,
            r2: r2,
            r3: r3,
            r4: r4,
            r5: r5,
            r6: r6,
            r7: r7,
            r8: r8,
            r9: r9,
            r10: r10,
            r11: r11,
            r12: r12,
            r13: r13,
            r14: r14,
            r15: r15,
            r16: r16,
            regI: regI
        }
    }
}

struct Chip8 {
    memory: Memory,
    reg_delay: DelayTimerRegister,
    reg_sound: SoundTimerRegister,
    pc: ProgramCounter,
    sp: StackPointer
}

impl Chip8 {
    fn new() -> Chip8 {
        let memory = Memory::new();
        let reg_delay = DelayTimerRegister{current: 0};
        let reg_sound = SoundTimerRegister{current: 0};
        let pc = ProgramCounter{count: 0};
        let sp = StackPointer{address: 0};

        Chip8 {
            memory: memory,
            reg_delay: reg_delay,
            reg_sound: reg_sound,
            pc: pc,
            sp: sp
        }
    }
    fn clear_screen() {
        
    }

    fn jump_to_address() {
        
    }

    fn jump_to_machine_code() {
        
    }

    fn interpret() {
        
    }
}

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
