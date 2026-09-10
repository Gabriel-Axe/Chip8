use rand::{Rng, rng};

use crate::{chip8::Chip8, debug_printer::DebugPrinter, memory::Memory, register::{Register, RegisterI}};

/// Chip 8 CPU
pub struct CPU {

    /// Array with 16 Registers, used in
    /// instructions that use Vx and Vy
    registers: [Register; 16],

    /// Program Counter
    pc: u16, 

    /// Stack Pointer
    sp: u8,

    /// Flag Register (?)
    VF: Register,

    /// Forgot the Name Register
    regI: RegisterI,

    /// Forgot the Name Register II
    dt_reg: Register,

    /// Sound Register
    st_reg: Register,
}

impl CPU {
    pub fn new() -> Self {
        DebugPrinter::log_info("create CPU".to_string());
        CPU {
            registers: [Register::new(); 16],
            VF: Register::new(),

            regI: RegisterI { data: 0 },

            dt_reg: Register::new(),
            st_reg: Register::new(),

            pc: 0x200,
            sp: 0,
        }
    }

    fn log_opcode(&self, code: String) {
        DebugPrinter::log_state(format!("opcode: {}", code));
    }

    pub fn set_vf_value(&mut self, active: bool) {
        if active {
            self.VF.data = 1;
            return;
        }
        self.VF.data = 0;
    }

    pub fn get_vf_value(&self) -> u8 {
        self.VF.data
    }

    fn jump_to_address(&mut self, address: u16) {
        self.log_opcode("JP (1)".to_string());
        // Chip8::log_chip8_action(format!("jump to 0x{:04X}", address)); 
        self.set_program_counter_to_address(address);
        self.increment_pc();
    }

    fn jump_offset_by_v0(&mut self) {
        self.log_opcode("JP (B)".to_string());
        let v0_data = self.get_register_data(0);
        let mut cur_addr = self.get_pc_address();
        self.jump_to_address(cur_addr + v0_data as u16);
    }

    // fn skip_instruction_if_vx_equal_keyboard_pressed(&mut self, reg_id: u8) {
    //     self.log_opcode("SKP".to_string());
    //     let vx_data = self.get_register_data(reg_id);
    //     if self.keys.contains(&vx_data) {
    //         self.increment_pc();
    //     }
    // }

    // fn skip_instruction_if_vx_not_equal_keyboard_pressed(&mut self, reg_id: u8) {
    //     self.log_opcode("SKNP".to_string());
    //     let vx_data = self.get_register_data(reg_id);
    //     if !self.keys.contains(&vx_data) {
    //         self.increment_pc();
    //     }
    // }

    fn skip_instruction_if_equal(&mut self, reg_id: u8, value: u8) {
        let register_data = self.get_register_data(reg_id);
        if register_data == value {
            self.increment_pc();
        }
    }

    fn skip_instruction_if_nequal(&mut self, reg_id: u8, value: u8) {
        let register_data = self.get_register_data(reg_id);
        if register_data != value {
            self.increment_pc();
        }
    }

    fn set_value_of_pc(&mut self, value: u16) {
        self.set_program_counter_to_value(value);
    }

    fn get_pc_address(&self) -> u16 {
        self.get_pc_address()
    }

    fn value_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        self.log_opcode("SE".to_string());
        let data = self.get_register_data(reg_id);

        if data == compared_val {
            self.increment_pc();
        }
    }

    pub fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
        self.log_opcode("SE".to_string());
        let register_1_data = self.get_register_data(reg_id_1);
        let register_2_data = self.get_register_data(reg_id_2);

        if register_1_data != register_2_data {
            self.increment_pc();
        }
    }

    fn value_not_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        self.log_opcode("SNE".to_string());
        let register_data = self.get_register_data(reg_id);
        if register_data != compared_val {
            self.increment_pc();
        }
    }

    fn byte_skip_instruction(&self) {
        
    }

    pub fn store_in_register_vx_val(&mut self, reg_id: u8, value: u8) {
        self.log_opcode("LD".to_string());
        let mut register_data = self.get_register_data(reg_id);
        register_data = value;
    }

    fn add_byte_operation(&mut self, reg_id: u8, value: u8) {
        self.log_opcode("ADD".to_string());
        let mut register_data = self.get_register_data(reg_id);
        let data = register_data;
        register_data = data + value;
    }

    fn and_number_to_random_value(&mut self, reg_id: u8, and_val: u8) {
        let mut rng = rng();

        let mut register_data = self.get_register_data(reg_id);
        let val = rng.next_u32() as u8;
        let result_val = and_val & val;
        register_data = result_val;
    }

    fn store_from_register_x_into_y(&mut self, reg_x_id: u8, reg_y_id: u8) {
        let data = self.get_register_data(reg_x_id);
        self.set_register_data(reg_y_id, data);
    }

    pub fn add_value_to_regixer_vx(&mut self, value: u8, reg_id: u8) {
        let data = self.get_register_data(reg_id);
        let mut new_data: u8 = 0;
        let temp: u16 = (data + value) as u16;
        if temp > 255 {
            println!("value > 255: {}", temp);
            // println!("value: {}", temp);
            new_data = (temp - 255) as u8;
        }
        else {
            let temp: u8 = (data + value) as u8;
            println!("value < 255: {}", temp);
            new_data = temp;
        }
        self.set_register_data(reg_id, new_data);
    }

    pub fn store_in_register_i_value(&mut self, value: u16) {
        self.set_register_i(value);
    }


    // fn store_from_register_y_into_x(&mut self, reg_x_id: u8, reg_y_id: u8) {
    //     let data = self.get_register_data(reg_y_id);
    //     self.set_register_data(reg_x_id, data);
    // }

    fn set_delay_timer_value_at_vx(&mut self, reg_id: u8) {
        let val = self.dt_reg.data;
        let register = self.set_register_data(reg_id, self.dt_reg.data);
    }

    fn set_delay_timer_value(&mut self, value: u8) {
        self.log_opcode("LD".to_string());
        self.dt_reg.data = value;
    }

    fn set_sound_timer_value(&mut self, value: u8) {
        self.st_reg.data = value;
    }

    fn call_address(&self) {
    }

    fn increment_vx_to_i(&mut self, reg_id: u8) {
        let reg_data = self.get_register_data(reg_id);
        let i_data = self.regI.data;
        self.set_register_i(i_data + reg_data as u16);
    }

    // fn store_registers_in_memory_up_to_vx(&mut self, up_to: u8) {
    //     let addr = self.regI.data;
    //     for i in 0..up_to {
    //         let reg_data =self.get_register_data(i);
    //         self.memory.set_in_address((addr + i as u16), reg_data);
    //     }
    // }

    fn store_bcd_of_vx(&mut self, reg_id: u8) {
        let val = self.get_register_data(reg_id);
        let hundreds = (val / 100) % 10;
        let tens = (val / 10) % 10;
        let units = val % 10;
    }

    // fn call_subroutine_at_address(&mut self, address: u16) {
    //     self.increment_sp();
    //     let sp_val = self.get_sp() as u16;
    //     self.stack[sp_val] = self.get_pc();
    //     self.set_program_counter_to_address(address);
    // }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&mut self, value: u16) {
        self.log_opcode("LD (A)".to_string());
        self.regI.data = value;
    }

    // fn set_keypress_at_vx(&mut self, reg_id: u8) {
    //     let key_pressed_vec = self.display.get_keys_pressed();
    //     let key_pressed = key_pressed_vec
    //         .first()
    //         .unwrap_or_else(|| {
    //             panic!("Could not get the first key pressed")
    //         });
    //     let key = self.keyboard_to_chip8_keyboard(*key_pressed);
    //     self.load_in_register(reg_id, key);
    // }

    // fn keyboard_to_chip8_keyboard(&mut self, key: Key) -> u8 {
    //     match key {
    //         Key::NumPad0 => KEY_0,
    //         Key::NumPad1 => KEY_1,
    //         Key::NumPad2 => KEY_2,
    //         Key::NumPad3 => KEY_3,
    //         Key::NumPad4 => KEY_4,
    //         Key::NumPad5 => KEY_5,
    //         Key::NumPad6 => KEY_6,
    //         Key::NumPad7 => KEY_7,
    //         Key::NumPad8 => KEY_8,
    //         Key::NumPad9 => KEY_9,
    //         _ => { return 0; }
    //     }
    // }

    // fn match_load_instruction(&self, load_type: u16, nibble_3: u16, nibble_2: u16, nibble_1: u16) {
    //     match load_type {
    //            6 => {
    //                let value = join_2_nibbles_into_u8(nibble_2 as u8, nibble_1 as u8);
    //                self.load_in_register(nibble_3 as u8, value);
    //            },
    //             8 => {
    //                 self.store_from_register_y_into_x(nibble_2 as u8, nibble_3 as u8);
    //             },
    //            }
    //        }   

    fn log_cpu_action(action: String) {
        DebugPrinter::log_action("cpu".to_string(), action);
    }

    pub fn get_register_data(&mut self, reg_id: u8) -> u8 {
            let len = self.registers.len();
            let reg_id = reg_id as usize;
            if reg_id > len || 0 > reg_id {
                panic!("Invalid register ID: {}", reg_id);
            }

            self.registers[reg_id].data
        }

    pub fn fetch_instruction_in_memory(&mut self, memory: &Memory) -> u8 {
        CPU::log_cpu_action("fetch instruction".to_string());
        let instruction = *memory.get_data_in_address(self.pc as usize);
        self.increment_pc();
        DebugPrinter::log_state(format!("instruction: 0x{:02X}", instruction));
        instruction
    }

    pub fn set_register_data(&mut self, reg_id: u8, val: u8) {
        let reg_id = reg_id as usize;
        if reg_id > self.registers.len() || 0 > reg_id {
            panic!("Invalid register ID: {}", reg_id);
        }

        self.registers[reg_id].data = val;
    }

    pub fn set_program_counter_to_address(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn set_program_counter_to_value(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn increment_sp(&mut self) {
        self.sp += 1;
    }

    pub fn decrement_sp(&mut self) {
        self.sp -= 1;
    }

    pub fn get_sp(&mut self) -> u8 {
        self.sp
    }

    pub fn get_pc(&mut self) -> u8 {
        self.sp
    }

    pub fn increment_pc(&mut self) {
        CPU::log_cpu_action("increment PC".to_string());
        self.pc += 1;
    }

    pub fn or_values(self, x: u8, y: u8) -> u8 {
        x | y
    }

    pub fn and_values(self, x: u8, y: u8) -> u8 {
        x & y
    }

    pub fn xor_values(self, x: u8, y: u8) -> u8 {
        let v1 = self.nand(!x, y);
        let v2 = self.nand(x, !y);
        self.nand(v1, v2)
    }

    pub fn add_values(&mut self, x: u8, y: u8) {
        let vy_data = self.get_register_data(y);
        let vx_data = self.get_register_data(x);

        if (vx_data + vy_data) > 255 {
            self.VF.data = 1;
        }

        let val = vx_data + vy_data;
        self.set_register_data(x, val);
    }

    pub fn sub_values(&mut self, x: u8, y: u8) {
        let mut vx = self.get_register_data(x);
        let vy = self.get_register_data(y);
        let mut vf = self.VF;

        if vx > vy {
            vf.data = 1;
        }

        let val = vy - vx;
        vx = val;
    }

    // pub fn store_delay_timer_in_vx(&mut self, reg_id: u8) {
    //     let mut register = self.get_register_data(reg_id);
    //     register.data = self.dt_reg.data;
    // }

    pub fn store_vx_in_delay_timer(&mut self, reg_id: u8) {
        self.set_register_data(reg_id, self.dt_reg.data);
    }

    // pub fn add_vx_to_register_i(&mut self, reg_id: u8) {
    //     let mut register = self.get_register_data(reg_id);
    //     let i_val = self.regI.data;
    //     self.regI.data = i_val + register.data as u16;
    // }

    // pub fn ld_store_in_memory(&mut self, memory: Memory) {
    //     let cur_addr = self.regI.data;
    //     for id_r in 0..15 {
    //         let reg = self.get_register_data(id_r);
    //         memory.data[cur_addr] = reg.data;
    //         cur_addr += 1;
    //     }
    // }

    // pub fn ld_read_in_memory(&mut self, memory: Memory) {
    //     let cur_addr = self.regI.data;
    //     for id_r in 0..15 {
    //         let reg = self.get_register_data(id_r);
    //         reg.data = memory.data[cur_addr];
    //     }
    // }

    pub fn store_vx_in_sound_timer(&mut self, reg_id: u8) {
        self.set_register_data(reg_id, self.st_reg.data);
    }

    // pub fn subn_values(&mut self, x: u8, y: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let vy = self.get_register_data(y);
    //     let mut vf = self.VF;
    // //
    //     if vx.data > vy.data {
    //         vf.data = 1;
    //     }
    // //
    //     let val = vx.data - vy.data;
    //     vx.data = val;
    // }

    // pub fn shr_values(&mut self, x: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let mut vf = self.VF;
    // //
    //     if vx.data & 1 == 1 {
    //         vf.data = 1;
    //     } else {
    //         vf.data = 0;
    //     }
    // //
    //     vx.data = vx.data / 2;
    // }

    pub fn return_from_subroutine(&mut self, memory: Memory) {
        let addr = memory.get_data_in_address(self.sp as usize);
        self.set_program_counter_to_address(*addr as u16);
        // WARN: wth is wrong wwith these methods wthat expect u16 and i sending a u8?
        self.decrement_sp();
    }

    // pub fn shl_values(&mut self, x: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let mut vf = self.VF;
    //
    //     if vx.data & 1 == 1 {
    //         vf.data = 1;
    //     } else {
    //         vf.data = 0;
    //     }
    //
    //     vx.data = vx.data * 2;
    // }

    fn nand(&self, x: u8, y: u8) -> u8 {
        !(x & y)
    }
}
