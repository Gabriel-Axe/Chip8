use crate::{cpu::CPU, memory::Memory, util::{get_instruction_nibble, join_2_nibbles_into_u8, join_3_nibbles_into_u8}};

use rand::{Rng, RngExt, rng};

pub struct Chip8 {
    cpu: CPU,
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 { 
        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new()
        }
    }

    fn clear_screen(&self) {
        
    }

    fn return_from_subroutine(&self) {
    }

    fn jump_to_address(&mut self, address: u16) {
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_offset_by_v0(&mut self) {
        let mut cur_addr = self.get_pc_address();
        let v0 = self.cpu.get_vx_register_by_id(0);
        self.jump_to_address(cur_addr + v0.data as u16);
    }

    fn skip_instruction_if_equal(&mut self, reg_id: u8, value: u8) {
        let register = self.cpu.get_vx_register_by_id(reg_id);
        if register.data == value {
            self.cpu.increment_pc();
        }
    }

    fn skip_instruction_if_nequal(&mut self, reg_id: u8, value: u8) {
        let register = self.cpu.get_vx_register_by_id(reg_id);
        if register.data != value {
            self.cpu.increment_pc();
        }
    }

    fn set_value_of_pc(&mut self, value: u16) {
        self.memory.pc = value;
    }

    fn reset_pc(&mut self) {
        self.memory.pc = 0;
    }

    fn get_pc_address(&self) -> u16 {
        self.memory.pc
    }

    fn value_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        let register = ;

        if register.data == compared_val {
            self.increment_pc();
        }
    }

    fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
        let register_1 = self.cpu.get_vx_register_by_id(reg_id_1);
        let register_2 = self.cpu.get_vx_register_by_id(reg_id_2);

        if register_1.data != register_2.data {
            self.increment_pc();
        }
    }

    fn value_not_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        let register = self.cpu.get_vx_register_by_id(reg_id);
        if register.data != compared_val {
            self.increment_pc();
        }
    }

    fn byte_skip_instruction(&self) {
        
    }

    fn set_value_at_register(&mut self, reg_id: u8, value: u8) {
        let mut register = self.cpu.get_vx_register_by_id(reg_id);
        register.data = value;
    }

    fn add_byte_operation(&mut self, reg_id: u8, value: u8) {
        let mut register = self.cpu.get_vx_register_by_id(reg_id);
        let data = register.data;
        register.data = data + value;
    }

    fn and_number_to_random_value(&mut self, reg_id: u8, and_val: u8) {
        let mut rng = rng();

        let mut register = self.cpu.get_vx_register_by_id(reg_id);
        let val = rng.next_u32() as u8;
        let result_val = and_val & val;
        register.data = result_val;
    }

    fn display_spryte(&self) {
        
    }

    fn store_from_register_x_into_y(&self) {
        
    }

    fn call_address(&self) {
    }

    fn call_subroutine_at_address(&mut self, address: u16) {
        self.cpu.increment_sp();
        self.memory.stack[self.cpu.get_sp()] = self.cpu.get_pc();
        self.cpu.set_program_counter_to_address(address);
    }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&mut self, value: u16) {
        self.memory.regI.data = value;
    }

    fn read_instruction(&mut self, mut instruction: u16) {
        let inst_type = get_instruction_nibble(instruction);

        let nibble_3 = instruction >> 3;
        let nibble_2 = instruction >> 2;
        let nibble_1 = instruction >> 1;

        match inst_type {
            0 => self.clear_screen(),
            1 => self.jump_to_address(),
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
            12 => self.and_number_to_random_value(),
            13 => self.display_spryte(),
            _ => return
        }
        self.increment_pc();
    }

    pub fn interpret(&mut self, instructions: &Vec<u16>) {
        for &inst in instructions {
            self.read_instruction(inst);
        }
    }
}
