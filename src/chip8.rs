use std::{any::Any, fs, io::Empty, num::ParseFloatError, path::PathBuf, thread::sleep, time::Duration};

use crate::{chip8::InstructionType::{ADD7, CALL, CALL2, CLS, DRW, JP_B, JP1, LD, LD_8, LD_A, LD_BYTE, LD6, RET, RND, RND_C, SE, SE_BYTE, SE3, SE5, SKNP, SKP, SNE, SNE4, SNE9}, cpu::CPU, debug_printer::DebugPrinter, display::Display, memory::Memory, util::{join_2_nibbles_into_u8, join_2_u8_into_u16, join_3_nibbles_into_u8}};

use minifb::{Key::{self, Key0}, Window, WindowOptions};
use rand::{Rng, rng};

const KEY_0: u8 = 0;
const KEY_1: u8 = 1;
const KEY_2: u8 = 2;
const KEY_3: u8 = 3;
const KEY_4: u8 = 4;
const KEY_5: u8 = 5;
const KEY_6: u8 = 6;
const KEY_7: u8 = 7;
const KEY_8: u8 = 8;
const KEY_9: u8 = 9;
const KEY_A: u8 = 10;
const KEY_B: u8 = 11;
const KEY_C: u8 = 12;
const KEY_D: u8 = 13;
const KEY_E: u8 = 14;
const KEY_F: u8 = 15;

pub struct Chip8 {
    cpu: CPU,
    display: Display,
    memory: Memory,
    keys: Vec<u8>,
}

pub enum InstructionType {
    CLS, 
    RET,
    JP1,
    JP_B,
    CALL2,
    RND_C,
    DRW,

    SE3,
    SNE4,
    SKP_E,
    SKNP_E,
    SE5,

    SHR8,

    OR8,
    AND8,
    XOR8,

    SUB8,
    SUBN8,

    SHL8,
    SNE9,

    ADD7,
    ADD8,
    ADD_F,

    LD6,
    LD8,
    LD_A,
    LD_F7,
    LD_FA,
    LD_F15,
    LD_F18,
    LD_F29,
    LD_F33,
    LD_F55,
    LD_F65,
}

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

        // let mut rom: Vec<u16> = Vec::new();
        // for i in (0..file.len()).step_by(2) {
        //     if i % 2 < file.len() {
        //     // if i + 1 < file.len() {
        //         let instruction = ((file[i] as u16) << 8) | (file[i + 1] as u16);
        //         rom.push(instruction);
        //     }
        // }
        let instructions = self.get_intructions_from_file(file);
        
        for (i, instruction) in instructions.iter().enumerate().step_by(2) {
            // WARN: How the heck does the chip8 instructions work?
            let addr = i as u16;
            let high = (instruction >> 8) as u8;
            let low = (instruction & 0xFF) as u8;
            self.memory.set_in_address(addr, high);
            self.memory.set_in_address(addr + 1, low);
        }

        DebugPrinter::log_info(format!("loaded ROM: {}", filename));
    }

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

    fn nibble_to_instruction_type(&self, instruction: u8) -> Option<InstructionType> {

        return match instruction {
            0 => Some(CLS),
            1 => Some(JP1),
            2 => Some(CALL2),
            3 => Some(SE3),
            4 => Some(SNE4),
            5 => Some(SE5),
            6 => Some(LD6),
            7 => Some(ADD7),
            8 => self.match_instruction_8(instruction),
            9 => Some(SNE9),
            10 => Some(LD_A),
            11 => Some(JP_B),
            12 => Some(RND_C),
            13 => Some(DRW),
            14 => self.match_instruction_E(instruction),
            15 => self.match_instruction_F(instruction),
            _ => return Option::None
        };
    }

    fn match_instruction_8(&self, instruction: u8) -> Option<InstructionType> {
        
    }

    fn match_instruction_E(&self, instruction: u8) -> Option<InstructionType> {
        
    }

    fn match_instruction_F(&self, instruction: u8) -> Option<InstructionType> {
        
    }

    pub fn new() -> Chip8 { 
        Chip8::log_chip8_action("create Chip 8".to_string());
        DebugPrinter::log_info("finish initialization".to_string());

        Chip8 {
            cpu: CPU::new(),
            display: Display::new(),
            memory: Memory::new(),
            keys: vec![
                KEY_0,
                KEY_1,
                KEY_2,
                KEY_3,
                KEY_4,
                KEY_5,
                KEY_6,
                KEY_7,
                KEY_8,
                KEY_9,
                KEY_A,
                KEY_B,
                KEY_C,
                KEY_D,
                KEY_E,
                KEY_F,
            ]
        }
    }

    fn clear_screen(&self) {
        self.log_opcode("CLS".to_string());
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
            .nibble_to_instruction_type(opcode as u8)
            .unwrap_or_else(|| {
                panic!("Could not unwrap opcode")
            });

        if op.type_id() != DRW.type_id() || op.type_id() != JP.type_id() {
            return
        }

        match op {
            // CLS => self.clear_screen(),
            RET => {
                let address = (&nibble_3 << 8) | (nibble_2 << 4) | nibble_1;
                self.jump_to_address(address) // WARN: Assuming I is for addresses...
            }
            JP => self.call_address(),
            CALL => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_equals_register_value(value, nibble_3 as u8);
            }
            SE_BYTE => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_not_equals_register_value(value as u8, nibble_3 as u8);
            }
            SNE => {
                self.compare_registers_values(nibble_2 as u8, nibble_3 as u8);
            }
            SE => {
                let value = join_2_nibbles_into_u8(nibble_1 as u8, nibble_2 as u8);
                self.load_in_register(nibble_3 as u8, value);
            }
            LD_BYTE => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.add_byte_operation(nibble_3 as u8, value);
            }
            LD_8 => {
                Chip8::log_chip8_action("oops, suposed to store in register".to_string());
                self.store_from_register_y_into_x(nibble_3 as u8, nibble_2 as u8);
                // let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                // self.value_equals_register_value(value, nibble_3 as u8);
            }
            LD => self.match_load_instruction(nibble_1, nibble_2, nibble_3),
            LD_A => {
                let value = join_3_nibbles_into_u8(nibble_1 as u8, nibble_2 as u8, nibble_3 as u8);
                self.set_register_i(value as u16);
            }
            JP_B => {
                self.jump_offset_by_v0()
            }
            // RND => self.and_number_to_random_value(),
             // 13 => self.draw_sprite(nibble_3 as u8, nibble_2 as u8, nibble_1 as u8),
            DRW => {
                // DebugPrinter::log_info("I REACHED HERE FOR RUSTY CRABE SAKE".to_string());
                // self.draw_pixel(1, 13);
                // self.draw_pixel(13, 1);
                self.display.draw_sprite(&self.memory, &mut self.cpu.regI, nibble_2 as u8, nibble_3 as u8, nibble_1 as u8);
                // sleep(Duration::new(1, 0));
            },
            SKP => {
                if nibble_2 == 9 && nibble_1 == 14 {
                    self.skip_instruction_if_vx_equal_keyboard_pressed(nibble_3 as u8);
                }
                if nibble_2 == 10 && nibble_1 == 1 {
                    self.skip_instruction_if_vx_not_equal_keyboard_pressed(nibble_3 as u8);
                }
            }
            SKNP => {
                if nibble_2 == 0 && nibble_1 == 7 {
                    self.set_delay_timer_value_at_vx(nibble_3 as u8);
                }

                if nibble_2 == 0 && nibble_1 == 10 {
                    self.set_keypress_at_vx(nibble_3 as u8);
                }

                if nibble_2 == 1 && nibble_1 == 5 {
                    self.set_delay_timer_value(nibble_3 as u8);
                }
                if nibble_2 == 1 && nibble_1 == 8 {
                    self.set_delay_timer_value(nibble_3 as u8);
                }
                if nibble_2 == 1 && nibble_1 == 14 {
                    self.increment_vx_to_i(nibble_3 as u8);
                }
                if nibble_2 == 3 && nibble_1 == 3 {
                    self.store_registers_in_memory_up_to_vx(nibble_3 as u8);
                }
                if nibble_2 == 5 && nibble_1 == 5 {
                    self.store_registers_in_memory_up_to_vx(nibble_3 as u8);
                }
            }
            _ => return
        }
    }

    pub fn run(&mut self) {
        DebugPrinter::log_info("initiate run".to_string());
        // self.display.draw_line_from_u8(1, 1, 0b10101010);
        // self.display.draw_line_from_u8(1, 2, 0b01010101);
        // self.display.draw_line_from_u8(1, 3, 0b10101010);
        // self.display.draw_line_from_u8(1, 4, 0b01010101);
           // self.display.draw_line_from_u8(1, 1, 0b00111100);
           // self.display.draw_line_from_u8(1, 2, 0b01000010);
           // self.display.draw_line_from_u8(1, 3, 0b10100101);
           // self.display.draw_line_from_u8(1, 4, 0b10000001);
           // self.display.draw_line_from_u8(1, 5, 0b10100101);
           // self.display.draw_line_from_u8(1, 6, 0b10011001);
           // self.display.draw_line_from_u8(1, 7, 0b01000010);
           // self.display.draw_line_from_u8(1, 8, 0b00111100);
            // self.display.draw_line_from_u8(1, 1, 0b11111111);
            // self.display.draw_line_from_u8(1, 2, 0b00000000);
            // self.display.draw_line_from_u8(1, 3, 0b11111111);
            // self.display.draw_line_from_u8(1, 4, 0b00000000);
            // self.display.draw_line_from_u8(1, 5, 0b11111111);
            // self.display.draw_line_from_u8(1, 6, 0b00000000);
            // self.display.draw_line_from_u8(1, 7, 0b11111111);
            // self.display.draw_line_from_u8(1, 8, 0b00000000);
        while self.display.is_open() && !self.display.is_key_down(Key::Escape) {
            Chip8::log_chip8_action("start run".to_string());
            let instr_1 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;
            let instr_2 = self.cpu.fetch_instruction_in_memory(&self.memory) as u16;
            let instruction = (instr_1 << 8) | instr_2;
            Chip8::log_chip8_action(format!("join  0x{:04X} to 0x{:04X} into 0x{:0$X}", instr_1 as usize, instr_2, instruction));
            self.read_instruction(&instruction);
            self.update();
            //
            // // let keys = self.display.get_keys_pressed();
            // // for key in keys {
            // //     println!("{:?}", key);
            // // }
        }
    }

    fn update(&mut self) {
        self.display.update();
    }

    pub fn get_nibble(&self, instruction: &u16, position: u8) -> u16 {
        let position = (position - 1) * 4;
        (instruction >> position) & 15
    }

    pub fn log_memory_contents(&self) {
        self.memory.log_contents();
    }
}

#[cfg(test)
mod tests {
    use super::*;

    #[test]
    fn () {
    }
}
