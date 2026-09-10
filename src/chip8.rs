use std::{any::Any, fs, io::Empty, num::ParseFloatError, path::PathBuf, thread::sleep, time::Duration};

use crate::{cpu::CPU, debug_printer::DebugPrinter, display::Display, memory::Memory, opcode_handler::Instruction::{self, CLEAR_SCREEN_0}, util::{join_2_nibbles_into_u8, join_2_u8_into_u16, join_3_nibbles_into_u8}};

use minifb::{Key::{self, Key0}, Window, WindowOptions};
use rand::{Rng, rng};


pub struct Chip8 {
    cpu: CPU,
    display: Display,
    memory: Memory,
}

/// The Chip 8 has many instructions names that are reused in between codes. 
/// For example, 0xAnnn is a LD instruction, but 0x8nnn, 0xFn15 and others also
/// have a LD type instruction.
/// Perhaps I could do a better job at filtering the LD type in the F instructions,
/// but this suffices for now
// pub enum InstructionType {
//     CLS, 
//     RET,
//     JP1,
//     JP_B,
//     CALL2,
//     RND_C,
//     DRW,
//     SE3,
//     SNE4,
//     SKP_E,
//     SKNP_E,
//     SE5,
//     SHR8,
//     OR8,
//     AND8,
//     XOR8,
//     SUB8,
//     SUBN8,
//     SHL8,
//     SNE9,
//     ADD7,
//     ADD8,
//     ADD_F,
//     LD6,
//     LD8,
//     LD_A,
//     LD_F7,
//     LD_FA,
//     LD_F15,
//     LD_F18,
//     LD_F29,
//     LD_F33,
//     LD_F55,
//     LD_F65,
// }

impl Chip8 {

    fn log_chip8_action(action: String) {
        DebugPrinter::log_action("chip8".to_string(), action);
    }

    pub fn load_rom(&mut self, path: &str, filename: &str) {
        Chip8::log_chip8_action("Loading ROM".to_string());
        let mut path = PathBuf::from(path);
        path.push(filename);

        let file  = fs::read(&path)
            .unwrap_or_else(|err| {
                panic!("Could not load ROM: {}, reason: {}", path.display(), err)
            });

        let instructions = self.get_intructions_from_file(file);
        
        for (i, instruction) in instructions.iter().enumerate().step_by(2) {
            let addr = i as u16;
            let high = (instruction >> 8) as u8;
            let low = (instruction & 0xFF) as u8;
            self.memory.set_in_address(addr, high);
            self.memory.set_in_address(addr + 1, low);
        }

        DebugPrinter::log_info(format!("loaded ROM: {}", filename));
    }

    /// Reads the suplied file contents and stores it
    /// in memory, actually, no it just gets the instructions from it
    /// ill do this later
    fn get_intructions_from_file(&self, rom: Vec<u8>) -> Vec<u16> {

        let mut temp: u8 = 0;
        let mut instructions: Vec<u16> = Vec::new();

        for (i, instruction) in rom.iter().enumerate() {
            if i % 2 == 1 {
                let stored = join_2_u8_into_u16(temp, *instruction);
                instructions.push(stored);
                temp = 0;
            }

            temp = *instruction;
        }

        instructions
    }

    fn nibble_to_instruction_type(&self, instruction: u16) -> Option<Instruction> {

        let x = (instruction >> 4 * 3) as u8;
        let y = (instruction >> 4 * 2 & 0xF) as u8;
        let kk = (instruction & 0xFF) as u8;
        let nnn = (instruction & 0xFFF) as u8;

        return match instruction {
            0 => Some(CLEAR_SCREEN_0),
            6 => Some(Instruction::LOAD_REGISTER_VX_WITH_VALUE_6 { x: x, value: kk }),
            7 => Some(Instruction::ADD_VALUE_TO_REGISTER_7 { x: x, value: kk }),
            10 => Some(Instruction::LOAD_INDEX_REGISTER_WITH_VALUE_A { value: nnn }),
            13 => Some(Instruction::DRAW_D { x: x, y: y, n: (instruction & 0xF) as u8 }),
            _ => return Option::None
            // 0 => Some(CLS),
            // 1 => Some(JP1),
            // 2 => Some(CALL2),
            // 3 => Some(SE3),
            // 4 => Some(SNE4),
            // 5 => Some(SE5),
            // 6 => Some(LD6),
            // 7 => Some(ADD7),
            // 8 => self.match_instruction_8(instruction),
            // 9 => Some(SNE9),
            // 10 => Some(LD_A),
            // 11 => Some(JP_B),
            // 12 => Some(RND_C),
            // 13 => Some(DRW),
        };
    }

    // fn match_instruction_8(&self, instruction: u8) -> Option<InstructionType> {
    fn match_instruction_8(&self, instruction: u8) -> Option<()> {

        Option::None
    }

    pub fn new() -> Chip8 { 
        Chip8::log_chip8_action("create Chip 8".to_string());
        DebugPrinter::log_info("finish initialization".to_string());

        Chip8 {
            cpu: CPU::new(),
            display: Display::new(),
            memory: Memory::new(),
        }
    }

    fn clear_screen(&self) {
    }

    fn read_instruction(&mut self, mut instruction: &u16) {

        Chip8::log_chip8_action("mask opcode".to_string());
        let opcode = self.get_nibble(instruction, 4);
        let nibble_3 = self.get_nibble(instruction, 3);
        let nibble_2 = self.get_nibble(instruction, 2);
        let nibble_1 = self.get_nibble(instruction, 1);
        DebugPrinter::log_state(format!(
            "instruction: 0x{:04X}, nibbles: n1={:X}, n2={:X}, n3={:X}, n4={:X}",
            instruction,
            opcode,
            nibble_3,
            nibble_2,
            nibble_1
        ));

        let op = self
            .nibble_to_instruction_type(opcode)
            .unwrap_or_else(|| {
                panic!("Could not unwrap opcode")
            });

        match op {
            _ => return
        }
    }

    pub fn run(&mut self) {
        DebugPrinter::log_info("initiate run".to_string());
        while self.display.is_open() && !self.display.is_key_down(Key::Escape) {
            Chip8::log_chip8_action("start run".to_string());
            let instr_1 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;
            let instr_2 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;
            let instruction = (instr_1 << 8) | instr_2;
            Chip8::log_chip8_action(format!("join  0x{:04X} to 0x{:04X} into 0x{:0$X}", instr_1 as usize, instr_2, instruction));
            self.read_instruction(&instruction);
            self.display.update();
        }
    }

    pub fn get_nibble(&self, instruction: &u16, position: u8) -> u16 {
        let position = (position - 1) * 4;
        (instruction >> position) & 15
    }
}
