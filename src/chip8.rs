use std::{any::Any, fmt::format, fs, io::Empty, num::ParseFloatError, path::PathBuf, thread::sleep, time::Duration};

use crate::{cpu::CPU, debug_printer::{self, DebugPrinter}, display::Display, file_handler::RomHandler, memory::{Memory, PROGRAM_START_OFFSET}, opcode_handler::Instruction::{self, ADD_VALUE_TO_REGISTER_7, CLEAR_SCREEN_0, DRAW_D, LOAD_INDEX_REGISTER_WITH_VALUE_A, LOAD_REGISTER_VX_WITH_VALUE_6, UNKNOWN}, util::{join_2_nibbles_into_u8, join_3_nibbles_into_u8}};

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

    /// Reads the suplied file contents and stores it
    /// in memory, actually, no it just gets the instructions from it
    /// ill do this later
    fn get_intructions_from_file(&self, rom: Vec<u8>) -> Vec<u16> {

        let mut temp: u8 = 0;
        let mut instructions: Vec<u16> = Vec::new();

        for (i, instruction) in rom.iter().enumerate() {
            if i % 2 == 1 {
                let temp: u16 = (temp << 4) as u16;
                let stored: u16 = (temp | *instruction as u16);
                instructions.push(stored);
                let temp: u8 = 0;
            }

            temp = *instruction;
        }

        instructions
    }

    fn instruction_to_type(&self, instruction: u16) -> Option<Instruction> {

        let opcode = (instruction >> 4 * 3) as u8;
        let x = (instruction >> 4 * 2 & 0xF) as u8;
        let y = (instruction >> 4 * 1 & 0xF) as u8;
        let kk = (instruction & 0xFF) as u8;
        let nnn = (instruction & 0xFFF) as u16;

        DebugPrinter::log_state(format!("instruction op: {:0X} x: {:0X} y: {:0X} kk: {:02X} nnn: {:03X} full: {:04X}", opcode, x, y, kk, nnn, instruction));

        return match opcode {
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

    pub fn new(rom: &str, path: Option<&str>) -> Chip8 { 
        Chip8::log_chip8_action("create Chip 8".to_string());
        DebugPrinter::log_info("finish initialization".to_string());

        let mut chip8 = Chip8 {
            cpu: CPU::new(),
            display: Display::new(),
            memory: Memory::new(),
            rom_handler: RomHandler::new(rom, path)
        };

        chip8.rom_handler.load_rom(&mut chip8.memory);
        chip8
    }

    fn clear_screen(&mut self) {
        self.display.clear_screen();
    pub fn set_folder(&mut self, path: &str) {
        self.rom_handler.set_folder(path);
        self.rom_handler.load_rom(&mut self.memory);
    }

    fn read_instruction(&mut self, mut instruction: &u16) {
        // let opcode = self.get_nibble(instruction, 4);
        //
        // let nibble_3 = self.get_nibble(instruction, 3);
        // let nibble_2 = self.get_nibble(instruction, 2);
        // let nibble_1 = self.get_nibble(instruction, 1);

        // DebugPrinter::log_state(format!(
        //     "instruction: 0x{:04X}, nibbles: n1={:X}, n2={:X}, n3={:X}, n4={:X}",
        //     instruction,
        //     opcode,
        //     nibble_3,
        //     nibble_2,
        //     nibble_1
        // ));

        let op = self
            .instruction_to_type(*instruction)
            .unwrap_or_else(|| { UNKNOWN });

        match op {
            CLEAR_SCREEN_0 => self.clear_screen(),
            LOAD_REGISTER_VX_WITH_VALUE_6 { x, value } => self.cpu.store_in_register_vx_val(x, value),
            ADD_VALUE_TO_REGISTER_7 { x, value } => self.cpu.add_value_to_register_vx(value, x),
            LOAD_INDEX_REGISTER_WITH_VALUE_A { value } => self.cpu.store_in_register_i_value(value),
            DRAW_D { x, y, n } => {
                Chip8::log_chip8_action("DRW operation".to_string());
                let conflict = self.display.draw_sprite(&self.memory, &mut self.cpu.get_regI_copy(), x, y, n);
                // WARN: Get here if conflict ocurred
                // if conflict {
                //     self.cpu.set_vf_value(true);
                // }
            },
            _ => {
                DebugPrinter::log_state(format!("unknown instruction type: {:04X}", *instruction));
                return
            }
        }
    }
    pub fn run(&mut self) {
        DebugPrinter::log_info("initiate run".to_string());

        while self.display.is_open() && !self.display.is_key_down(Key::Escape) {


            let instr_1 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;
            let instr_2 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;

            let instruction = (instr_1 << 8) | instr_2;
            Chip8::log_chip8_action(format!("join  0x{:04X} to 0x{:04X} into 0x{:04X}", instr_1, instr_2, instruction));

            self.read_instruction(&instruction);
            self.cpu.log_state();
            self.display.update();
        }
    }

    pub fn get_nibble(&self, instruction: &u16, position: u8) -> u16 {
        let position = (position - 1) * 4;
        (instruction >> position) & 15
    }
}
