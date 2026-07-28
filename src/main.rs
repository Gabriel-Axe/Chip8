use core::num;
use std::{env, fs, io::Bytes, path::PathBuf, str::{FromStr, from_utf8}, vec};

struct Register {
    data: u16
}

impl Register {
    fn new() -> Self {
        Self { data: 0 }
    }
}

struct DelayTimerRegister {
    reg: Register
}

struct SoundTimerRegister {
    reg: Register
}

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

struct Memory {
    V1: Register,
    V2: Register,
    V3: Register,
    V4: Register,
    V5: Register,
    V6: Register,
    V7: Register,
    V8: Register,
    V9: Register,
    V10: Register,
    V11: Register,
    V12: Register,
    V13: Register,
    V14: Register,
    V15: Register,
    V16: Register,

    VF: Register,

    regI: Register,

    dt_reg: Register,
    st_reg: Register,
}

impl Memory {
    fn new() -> Self {

        Memory {
            V1: Register::new(),
            V2: Register::new(),
            V3: Register::new(),
            V4: Register::new(),
            V5: Register::new(),
            V6: Register::new(),
            V7: Register::new(),
            V8: Register::new(),
            V9: Register::new(),
            V10: Register::new(),
            V11: Register::new(),
            V12: Register::new(),
            V13: Register::new(),
            V14: Register::new(),
            V15: Register::new(),
            V16: Register::new(),

            VF: Register::new(),

            regI: Register::new(),

            dt_reg: Register::new(),
            st_reg: Register::new()
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
    fn clear_screen(self) {
        
    }

    fn return_from_subroutine(mut self) {
        self.sp.address -= 1;
    }

    fn jump_to_address(self) {
        
    }

    fn jump_to_machine_code(self) {
        
    }

    fn read_instruction(self, mut instruction: Instruction) {
    }

    fn interpret(self, mut bytes: Vec<u8>) {
        let instruction_type = bytes.pop().unwrap(); // WARN: DANGER, unsafe operation
        let instruction_data = bytes.pop().unwrap();
        let instruction = Instruction {inst_type: instruction_type, inst_data: instruction_data, };
        self.read_instruction(instruction);
    }

    // fn somethingelse() {
    //     for byte in bytes  {
    //         instruction = 
    //             match byte {
    //                 224 => self.clear_screen(),           
    //                 238 => self.return_from_subroutine(),
    //                 _ => break
    //             }
    //     }
    }

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
