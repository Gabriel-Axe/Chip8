use std::{fs, path::PathBuf};

use crate::{cpu::CPU, memory::Memory, util::{from_u8_rgb, from_u8_to_bin_color, get_instruction_nibble, join_2_nibbles_into_u8, join_3_nibbles_into_u8}};

use minifb::{Key::{self, Key0}, Window, WindowOptions};
use rand::{Rng, rng};

const WINDOW_WIDTH: usize = 1920 / 3;
const WINDOW_HEIGHT: usize = 1080 / 3;
const BUFFER_WIDTH: usize = 64;
const BUFFER_HEIGHT: usize = 32;

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
    rom: Vec<u16>,
}

impl Chip8 {

    pub fn load_rom(&mut self, filename: &str) {
        let mut path = PathBuf::from("roms/chip8-roms/games");
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
        
        self.rom = rom;
    }


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

        // let azure_blue = from_u8_rgb(0, 127, 255);
        // let red = from_u8_rgb(255, 127, 0);
        // let mut buffer: Vec<bool> = vec![false; BUFFER_WIDTH * BUFFER_HEIGHT];
        let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];
        // buffer[66] = from_u8_to_bin_color(255);
        // buffer[64] = from_u8_to_bin_color(255);
        // buffer[63] = from_u8_to_bin_color(255);
        // buffer[10] = from_u8_to_bin_color(255);
        // buffer[74] = from_u8_to_bin_color(255);

        // for i in 256..544 {
        //     buffer[i] = from_u8_to_bin_color(i as u8);
        // }
        // let mut aaa = 0;
        // for i in 256..544 {
        //     if aaa == 4 {
        //     aaa = 0;
        //     buffer[i] = from_u8_to_bin_color(i as u8);
        //     }
        //     aaa += 1;
        // }

        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: window,
            display_buffer: buffer,
            rom: Vec::new(),
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

    pub fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
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

    fn display_sprite(&mut self, x: u8, y: u8, n_bytes: u8) {
        let starting_addr = self.cpu.regI.data as u8;
        let mut buffer = self.display_buffer.clone();
        for addr in starting_addr..starting_addr + n_bytes {
            let mem_val = self.memory.data[addr as usize];
            let cur_vals = buffer[((x * y) + addr) as usize];
            let new_color = from_u8_to_bin_color(mem_val as u8);
            let next_vals = (cur_vals | new_color);
            if next_vals <= 0 {
                self.cpu.VF.data = 1;
            }
            else {
                self.cpu.VF.data = 1;
            }
            buffer[((x * y) + addr) as usize] = next_vals;
        }

        self.display_buffer = buffer;
    }

    fn store_from_register_x_into_y(&self) {
        
    }

    fn set_delay_timer_value_at_vx(&mut self, reg_id: u8) {
        let val = self.cpu.dt_reg.data;
        let register = self.cpu.set_register_data(reg_id as usize, self.cpu.dt_reg.data);
    }

    fn set_delay_timer_value(&mut self, value: u8) {
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
            self.memory.data[(addr + i as u16) as usize] = reg_data as u16;
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
        self.memory.stack[sp_val as usize] = self.cpu.get_pc() as u16; // WARN: Not same byte
                                                                       // size
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&mut self, value: u16) {
        self.cpu.regI.data = value;
    }

    fn set_keypress_at_vx(&mut self, reg_id: u8) {
        // WARN: I must freeze execution until a
        // key is pressed
        let key_pressed_vec = self.display.get_keys_pressed(minifb::KeyRepeat::No);
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
            15 => {
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
        self.cpu.increment_pc();
    }

    pub fn run(&mut self) {
        while self.display.is_open() && !self.display.is_key_down(Key::Escape) {
            self.interpret();
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
        let mut copy_vec: Vec<u16> = Vec::new();
        copy_vec.resize(self.rom.len(), 0);
        copy_vec.copy_from_slice(self.rom.as_slice());
        for instruction in copy_vec {
            self.read_instruction(&instruction);
        }
    }

    pub fn draw_square(&mut self) {
        let start_x = 10;
        let end_x = 40;
        let start_y = 10;
        let end_y = 20;
        for x in start_x..end_x {
            for y in start_y..end_y {
                self.draw_pixel(x, y);
            }
        }
    }

    pub fn draw_h_line(&mut self) {
        let start_x = 10;
        let end_x = 40;
        let y = 32;
        for x in start_x..end_x {
            self.draw_pixel(x, y);
        }
    }

    pub fn draw_v_line(&mut self) {
        let start_y = 6;
        let end_y = 28;
        let x = 54;
        for y in start_y..end_y {
            self.draw_pixel(x, y);
        }
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        if x < 0 || x > BUFFER_WIDTH as u8 || y < 0 || y > BUFFER_HEIGHT as u8 {
            panic!("Pixel out of buffer");
        }
        let x_loc: usize = (x - 1) as usize;
        let y_loc: usize = (BUFFER_WIDTH * y as usize) - BUFFER_WIDTH;
        let loc = x_loc + y_loc;
        self.invert_pixel(loc);
    }

    fn invert_pixel(&mut self, loc: usize) {
        if self.display_buffer[loc as usize] != 0 {
            self.display_buffer[loc as usize] = 0;
        }
        self.display_buffer[loc as usize] = from_u8_to_bin_color(255);
    }
}
