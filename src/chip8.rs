use crate::{cpu::CPU, memory::Memory, util::{from_u8_rgb, get_instruction_nibble, join_2_nibbles_into_u8, join_3_nibbles_into_u8}};

use minifb::{Key, Window, WindowOptions};
use rand::{Rng, rng};

const WINDOW_WIDTH: usize = 1920 / 3;
const WINDOW_HEIGHT: usize = 1080 / 3;
const BUFFER_WIDTH: usize = 40;
const BUFFER_HEIGHT: usize = 30;

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
    memory: Memory,
    display: Window,
    // display_buffer: Vec<bool>,
    display_buffer: Vec<u32>,
    keys: Vec<u8>,
}

impl Chip8 {
    pub fn new() -> Chip8 { 
        let mut window = Window::new(
            "Chip 8",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("Could not create emulator window: {}", e)
            });

        window.set_target_fps(60);

        let azure_blue = from_u8_rgb(0, 127, 255);
        // let mut buffer: Vec<bool> = vec![false; BUFFER_WIDTH * BUFFER_HEIGHT];
        let mut buffer: Vec<u32> = vec![azure_blue; BUFFER_WIDTH * BUFFER_HEIGHT];

        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: window,
            display_buffer: buffer,
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

    // fn load_rom(filename: &str) -> Vec<u16> {
    //     // let file = fs::read(filename).expect("Could not load ROM");
    //     // let file_as_u16 = convert_bytes_u8_to_u16(&file).expect("Could not convert ROM to u16");
    //     // file_as_u16
    // }

    fn clear_screen(&self) {
        
    }

    fn jump_to_address(&mut self, address: u16) {
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_offset_by_v0(&mut self) {
        let v0_data = self.cpu.get_register_data(0);
        let mut cur_addr = self.get_pc_address();
        self.jump_to_address(cur_addr + v0_data as u16);
    }

    fn skip_instruction_if_vx_equal_keyboard_pressed(&mut self, reg_id: u8) {
        let vx_data = self.cpu.get_register_data(reg_id as usize);
        if self.keys.contains(&vx_data) {
            self.cpu.increment_pc();
        }
    }

    fn skip_instruction_if_vx_not_equal_keyboard_pressed(&mut self, reg_id: u8) {
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
        // let register = ;
        //
        // if register.data == compared_val {
        //     self.increment_pc();
        // }
    }

    fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
        let register_1_data = self.cpu.get_register_data(reg_id_1 as usize);
        let register_2_data = self.cpu.get_register_data(reg_id_2 as usize);

        if register_1_data != register_2_data {
            self.cpu.increment_pc();
        }
    }

    fn value_not_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        let register_data = self.cpu.get_register_data(reg_id as usize);
        if register_data != compared_val {
            self.cpu.increment_pc();
        }
    }

    fn byte_skip_instruction(&self) {
        
    }

    fn set_value_at_register(&mut self, reg_id: u8, value: u8) {
        let mut register_data = self.cpu.get_register_data(reg_id as usize);
        register_data = value;
    }

    fn add_byte_operation(&mut self, reg_id: u8, value: u8) {
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

    // fn display_sprite(&self, n_bytes: u8) {
    //     let starting_addr = self.cpu.regI.data;
    //     for addr in starting_addr..n_bytes  {
    //         let mem_val = self.memory.data[addr];
    //         self.display_buffer[mem_val] = 
    //     }
    // }

    fn store_from_register_x_into_y(&self) {
        
    }

    fn call_address(&self) {
    }

    fn call_subroutine_at_address(&mut self, address: u16) {
        self.cpu.increment_sp();
        let sp_val = self.cpu.get_sp() as u16;
        self.memory.stack[sp_val as usize] = self.cpu.get_pc() as u16; // WARN: Not same byte
                                                                       // size
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&mut self, value: u16) {
        self.cpu.regI.data = value;
    }

    fn read_instruction(&mut self, mut instruction: u16) {
        let inst_type = get_instruction_nibble(instruction);

        let nibble_3 = instruction >> 3;
        let nibble_2 = instruction >> 2;
        let nibble_1 = instruction >> 1;

        match inst_type {
            0 => self.clear_screen(),
            1 => self.jump_to_address(self.cpu.regI.data), // WARN: Assuming I is for addresses...
            2 => self.call_address(),
            3 => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_equals_register_value(value, nibble_3 as u8);
            }
            4 => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_not_equals_register_value(value as u8, nibble_3 as u8);
            }
            5 => {
                self.compare_registers_values(nibble_2 as u8, nibble_3 as u8);
            }
            6 => {
                let value = join_2_nibbles_into_u8(nibble_1 as u8, nibble_2 as u8);
                self.set_value_at_register(nibble_3 as u8, value);
            }
            7 => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.add_byte_operation(nibble_3 as u8, value);
            }
            9 => {
                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
                self.value_equals_register_value(value, nibble_3 as u8);
            }
            10 => {
                let value = join_3_nibbles_into_u8(nibble_1 as u8, nibble_2 as u8, nibble_3 as u8);
                self.set_register_i(value as u16);
            }
            11 => {
                self.jump_offset_by_v0()
            }
            // 12 => self.and_number_to_random_value(),
            // 13 => self.display_sprite(),
            14 => {
                if nibble_2 == 9 && nibble_1 == 14 {
                    self.skip_instruction_if_vx_equal_keyboard_pressed(nibble_3 as u8);
                }
                if nibble_2 == 10 && nibble_1 == 1 {
                    self.skip_instruction_if_vx_not_equal_keyboard_pressed(nibble_3 as u8);
                }
            }
            _ => return
        }
        self.cpu.increment_pc();
    }

    fn load_rom () {
        
    }

    pub fn run(&mut self) {
        while self.display.is_open() && !self.display.is_key_down(Key::Escape) {
            self.update();

            let keys = self.display.get_keys_pressed(minifb::KeyRepeat::No);
            for key in keys {
                println!("{:?}", key);
            }
        }
    }

    fn update(&mut self) {
        // let buffer: Vec<u32> = self.display_buffer.into();
        self.display
            .update_with_buffer(&self.display_buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Could not update display: {}", e)
            });
    }

    pub fn interpret(&mut self) {
        // for &inst in instructions {
        //     self.read_instruction(inst);
        // }
    }
}
