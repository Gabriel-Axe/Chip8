use std::{any::Any, fs, io::Empty, num::ParseFloatError, path::PathBuf, thread::sleep, time::Duration};

use crate::{chip8::InstructionType::{CALL, CLS, DRW, JP, JP_B, LD, LD_8, LD_A, LD_BYTE, RET, RND, SE, SE_BYTE, SKNP, SKP, SNE}, cpu::CPU, debug_printer::DebugPrinter, display::Display, memory::Memory, util::{from_u8_rgb, from_u8_to_bin_color, join_2_nibbles_into_u8, join_3_nibbles_into_u8}};

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
    JP,
    CALL,
    SE_BYTE,
    SNE,
    SE,
    LD_BYTE,
    ADD_BYTE,
    LD,
    LD_8,
    OR,
    AND,
    XOR,
    ADD,
    SUB,
    SHR,
    SUBN,
    SHL,
    SNE_9,
    LD_A,
    JP_B,
    RND,
    DRW,
    SKP,
    SKNP,
    LD_Fx07,
    LD_Fx0A,
    LD_Fx15,
    LD_Fx18,
    ADD_F,
    LD_Fx29,
    LD_Fx33,
    LD_Fx55,
    LD_Fx65,
}

impl Chip8 {

    fn log_chip8_action(action: String) {
        DebugPrinter::log_action("chip8".to_string(), action);
    }

    pub fn load_rom(&mut self, filename: &str) {
        Chip8::log_chip8_action("Loading ROM".to_string());
        // let mut path = PathBuf::from("roms/chip8-roms/programs");
        // let mut path = PathBuf::from("test_rom");
        let mut path = PathBuf::from("test_suite/bin");
        path.push(filename);

        let file  = fs::read(&path)
            .unwrap_or_else(|err| {
                panic!("Could not load ROM: {}, reason: {}", path.display(), err)
            });

        let mut rom: Vec<u16> = Vec::new();
        for i in (0..file.len()).step_by(2) {
            if i + 1 < file.len() {
                let instruction = ((file[i] as u16) << 8) | (file[i + 1] as u16);
                rom.push(instruction);
            }
        }
        
        // WARN: Possibly shows absolute path
        for (i, instruction) in rom.iter().enumerate().step_by(2) {
            let addr = i as u16;
            // let instr_1 = (instruction & 0b1111) as u8;
            // let instr_2 = (instruction >> 8) as u8;
            let high = (instruction >> 8) as u8;
            let low = (instruction & 0xFF) as u8;
            self.memory.set_in_address(addr, high);
            self.memory.set_in_address(addr + 1, low);
        }
        DebugPrinter::log_info(format!("loaded ROM: {}", filename));
    }

    // pub fn log_instructions(&self) {
    //     for instruction in self.rom.iter() {
    //         log::debug!("(ROM) instruction={:04X}", instruction);
    //     }
    // }

    fn nibble_to_instruction_type(&self, instruction: u8) -> Option<InstructionType> {

        return match instruction {
            0 => Some(CLS),
            1 => Some(RET),
            2 => Some(JP),
            3 => Some(CALL),
            4 => Some(SE_BYTE),
            5 => Some(SNE),
            6 => Some(SE),
            7 => Some(LD_BYTE),
            8 => Some(LD_8),
            9 => Some(LD), // TODO: Need to put 9 other instructions here
            10 => Some(LD_A),
            11 => Some(JP_B),
            12 => Some(RND),
            13 => Some(DRW),
            14 => Some(SKP),
            15 => Some(SKNP),
            _ => return Option::None
        };
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

    fn log_opcode(&self, code: String) {
        DebugPrinter::log_state(format!("opcode: {}", code));
    }

    fn clear_screen(&self) {
        self.log_opcode("CLS".to_string());
    }

    fn jump_to_address(&mut self, address: u16) {
        // WARN: Btw, this increments PC
        self.log_opcode("JP (1)".to_string());
        Chip8::log_chip8_action(format!("jump to 0x{:04X}", address)); 
        self.cpu.set_program_counter_to_address(address);
        self.cpu.increment_pc();
    }

    fn jump_offset_by_v0(&mut self) {
        self.log_opcode("JP (B)".to_string());
        let v0_data = self.cpu.get_register_data(0);
        let mut cur_addr = self.get_pc_address();
        self.jump_to_address(cur_addr + v0_data as u16);
    }

    fn skip_instruction_if_vx_equal_keyboard_pressed(&mut self, reg_id: u8) {
        self.log_opcode("SKP".to_string());
        let vx_data = self.cpu.get_register_data(reg_id as usize);
        if self.keys.contains(&vx_data) {
            self.cpu.increment_pc();
        }
    }

    fn skip_instruction_if_vx_not_equal_keyboard_pressed(&mut self, reg_id: u8) {
        self.log_opcode("SKNP".to_string());
        let vx_data = self.cpu.get_register_data(reg_id as usize);
        if !self.keys.contains(&vx_data) {
            self.cpu.increment_pc();
        }
    }

    fn skip_instruction_if_equal(&mut self, reg_id: u8, value: u8) {
        let register_data = self.cpu.get_register_data(reg_id as usize as usize);
        if register_data == value {
            self.cpu.increment_pc();
        }
    }

    fn skip_instruction_if_nequal(&mut self, reg_id: u8, value: u8) {
        let register_data = self.cpu.get_register_data(reg_id as usize);
        if register_data != value {
            self.cpu.increment_pc();
        }
    }

    fn set_value_of_pc(&mut self, value: u16) {
        self.cpu.set_program_counter_to_value(value);
    }

    fn reset_pc(&mut self) {
        self.cpu.reset_pc();
    }

    fn get_pc_address(&self) -> u16 {
        self.get_pc_address()
    }

    fn value_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        self.log_opcode("SE".to_string());
        // let register = ;
        //
        // if register.data == compared_val {
        //     self.increment_pc();
        // }
    }

    pub fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
        self.log_opcode("SE".to_string());
        let register_1_data = self.cpu.get_register_data(reg_id_1 as usize);
        let register_2_data = self.cpu.get_register_data(reg_id_2 as usize);

        if register_1_data != register_2_data {
            self.cpu.increment_pc();
        }
    }

    fn value_not_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        self.log_opcode("SNE".to_string());
        let register_data = self.cpu.get_register_data(reg_id as usize);
        if register_data != compared_val {
            self.cpu.increment_pc();
        }
    }

    fn byte_skip_instruction(&self) {
        
    }

    fn set_value_at_register(&mut self, reg_id: u8, value: u8) {
        self.log_opcode("LD".to_string());
        let mut register_data = self.cpu.get_register_data(reg_id as usize);
        register_data = value;
    }

    fn add_byte_operation(&mut self, reg_id: u8, value: u8) {
        self.log_opcode("ADD".to_string());
        let mut register_data = self.cpu.get_register_data(reg_id as usize);
        let data = register_data;
        register_data = data + value;
    }

    fn and_number_to_random_value(&mut self, reg_id: u8, and_val: u8) {
        let mut rng = rng();

        let mut register_data = self.cpu.get_register_data(reg_id as usize);
        let val = rng.next_u32() as u8;
        let result_val = and_val & val;
        register_data = result_val;
    }

    fn store_from_register_x_into_y(&mut self, reg_x_id: u8, reg_y_id: u8) {
        let data = self.cpu.get_register_data(reg_x_id as usize);
        self.cpu.set_register_data(reg_y_id as usize, data);
    }

    fn store_from_register_y_into_x(&mut self, reg_x_id: u8, reg_y_id: u8) {
        let data = self.cpu.get_register_data(reg_y_id as usize);
        self.cpu.set_register_data(reg_x_id as usize, data);
    }

    fn set_delay_timer_value_at_vx(&mut self, reg_id: u8) {
        let val = self.cpu.dt_reg.data;
        let register = self.cpu.set_register_data(reg_id as usize, self.cpu.dt_reg.data);
    }

    fn set_delay_timer_value(&mut self, value: u8) {
        self.log_opcode("LD".to_string());
        self.cpu.dt_reg.data = value;
    }

    fn set_sound_timer_value(&mut self, value: u8) {
        self.cpu.st_reg.data = value;
    }

    fn call_address(&self) {
    }

    fn increment_vx_to_i(&mut self, reg_id: u8) {
        let reg_data = self.cpu.get_register_data(reg_id as usize);
        let i_data = self.cpu.regI.data;
        self.set_register_i(i_data + reg_data as u16);
    }

    fn store_registers_in_memory_up_to_vx(&mut self, up_to: u8) {
        let addr = self.cpu.regI.data;
        for i in 0..up_to {
            let reg_data =self.cpu.get_register_data(i as usize);
            self.memory.set_in_address((addr + i as u16), reg_data);
        }
    }

    fn store_bcd_of_vx(&mut self, reg_id: u8) {
        let val = self.cpu.get_register_data(reg_id as usize);
        let hundreds = (val / 100) % 10;
        let tens = (val / 10) % 10;
        let units = val % 10;
    }

    fn call_subroutine_at_address(&mut self, address: u16) {
        self.cpu.increment_sp();
        let sp_val = self.cpu.get_sp() as u16;
        self.memory.stack[sp_val as usize] = self.cpu.get_pc(); // WARN: Not same byte
                                                                       // size
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&mut self, value: u16) {
        self.log_opcode("LD (A)".to_string());
        self.cpu.regI.data = value;
    }

    fn set_keypress_at_vx(&mut self, reg_id: u8) {
        // WARN: I must freeze execution until a
        // key is pressed
        let key_pressed_vec = self.display.get_keys_pressed();
        let key_pressed = key_pressed_vec
            .first()
            .unwrap_or_else(|| {
                panic!("Could not get the first key pressed")
            });
        // WARN: What do i do when 2 keys are pressed?
        let key = self.keyboard_to_chip8_keyboard(*key_pressed);
        self.set_value_at_register(reg_id, key);
    }

    fn keyboard_to_chip8_keyboard(&mut self, key: Key) -> u8 {
        match key {
            Key::NumPad0 => KEY_0,
            Key::NumPad1 => KEY_1,
            Key::NumPad2 => KEY_2,
            Key::NumPad3 => KEY_3,
            Key::NumPad4 => KEY_4,
            Key::NumPad5 => KEY_5,
            Key::NumPad6 => KEY_6,
            Key::NumPad7 => KEY_7,
            Key::NumPad8 => KEY_8,
            Key::NumPad9 => KEY_9,
            _ => { return 0; }
        }
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
                self.set_value_at_register(nibble_3 as u8, value);
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
            LD => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_equals_register_value(value, nibble_3 as u8);
            }
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

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        self.display.draw_pixel(x, y);
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
        // let buffer: Vec<u32> = self.display_buffer.into();
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
