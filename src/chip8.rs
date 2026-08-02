use crate::{memory::Memory, util::{get_instruction_nibble, join_2_nibbles_into_u8}};

pub struct Chip8 {
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 { 
        Chip8 {
            memory: Memory::new()
        }
    }

    fn clear_screen(&self) {
        
    }

    fn return_from_subroutine(&self) {
    }

    fn jump_to_address(&self) {
        
    }

    fn jump_offset_by_v0(&self) {
        
    }

    fn skip_instruction_if(&self) {
        
    }

    fn increment_pc(&mut self) {
        self.memory.pc += 1;
    }

    fn set_value_of_pc(&mut self, value: u16) {
        self.memory.pc = value;
    }

    fn reset_pc(&mut self) {
        self.memory.pc = 0;
    }

    fn value_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        let register = match reg_id {
            0 => &self.memory.V0,
            1 => &self.memory.V1,
            2 => &self.memory.V2,
            3 => &self.memory.V3,
            4 => &self.memory.V4,
            5 => &self.memory.V5,
            6 => &self.memory.V6,
            7 => &self.memory.V7,
            8 => &self.memory.V8,
            9 => &self.memory.V9,
            10 => &self.memory.V10,
            11 => &self.memory.V11,
            13 => &self.memory.V13,
            14 => &self.memory.V14,
            12 => &self.memory.V12,
            15 => &self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id),
        };

        if register.data == compared_val {
            self.increment_pc();
        }
    }

    fn compare_registers_values(&mut self, reg_id_1: u8, reg_id_2: u8) {
        let register_1 = match reg_id_1 {
            0 => &self.memory.V0,
            1 => &self.memory.V1,
            2 => &self.memory.V2,
            3 => &self.memory.V3,
            4 => &self.memory.V4,
            5 => &self.memory.V5,
            6 => &self.memory.V6,
            7 => &self.memory.V7,
            8 => &self.memory.V8,
            9 => &self.memory.V9,
            10 => &self.memory.V10,
            11 => &self.memory.V11,
            12 => &self.memory.V12,
            13 => &self.memory.V13,
            14 => &self.memory.V14,
            15 => &self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id_1),
        };

        let register_2 = match reg_id_2 {
            0 => &self.memory.V0,
            1 => &self.memory.V1,
            2 => &self.memory.V2,
            3 => &self.memory.V3,
            4 => &self.memory.V4,
            5 => &self.memory.V5,
            6 => &self.memory.V6,
            7 => &self.memory.V7,
            8 => &self.memory.V8,
            9 => &self.memory.V9,
            10 => &self.memory.V10,
            11 => &self.memory.V11,
            12 => &self.memory.V12,
            13 => &self.memory.V13,
            14 => &self.memory.V14,
            15 => &self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id_2),
        };

        if register_1.data != register_2.data {
            self.increment_pc();
        }
    }

    fn value_not_equals_register_value(&mut self, compared_val: u8, reg_id: u8) {
        let register = match reg_id {
            0 => &self.memory.V0,
            1 => &self.memory.V1,
            2 => &self.memory.V2,
            3 => &self.memory.V3,
            4 => &self.memory.V4,
            5 => &self.memory.V5,
            6 => &self.memory.V6,
            7 => &self.memory.V7,
            8 => &self.memory.V8,
            9 => &self.memory.V9,
            10 => &self.memory.V10,
            11 => &self.memory.V11,
            12 => &self.memory.V12,
            13 => &self.memory.V13,
            14 => &self.memory.V14,
            15 => &self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id),
        };

        if register.data != compared_val {
            self.increment_pc();
        }
    }

    fn byte_skip_instruction(&self) {
        
    }

    fn set_value_at_register(&mut self, reg_id: u8, value: u8) {
        let mut register = match reg_id {
            0 => &mut self.memory.V0,
            1 => &mut self.memory.V1,
            2 => &mut self.memory.V2,
            3 => &mut self.memory.V3,
            4 => &mut self.memory.V4,
            5 => &mut self.memory.V5,
            6 => &mut self.memory.V6,
            7 => &mut self.memory.V7,
            8 => &mut self.memory.V8,
            9 => &mut self.memory.V9,
            10 => &mut self.memory.V10,
            11 => &mut self.memory.V11,
            12 => &mut self.memory.V12,
            13 => &mut self.memory.V13,
            14 => &mut self.memory.V14,
            15 => &mut self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id),
        };

        register.data = value;
    }

    fn add_byte_operation(&mut self, reg_id: u8, value: u8) {
        let mut register = match reg_id {
            0 => &mut self.memory.V0,
            1 => &mut self.memory.V1,
            2 => &mut self.memory.V2,
            3 => &mut self.memory.V3,
            4 => &mut self.memory.V4,
            5 => &mut self.memory.V5,
            6 => &mut self.memory.V6,
            7 => &mut self.memory.V7,
            8 => &mut self.memory.V8,
            9 => &mut self.memory.V9,
            10 => &mut self.memory.V10,
            11 => &mut self.memory.V11,
            12 => &mut self.memory.V12,
            13 => &mut self.memory.V13,
            14 => &mut self.memory.V14,
            15 => &mut self.memory.V15,
            _ => panic!("Invalid register ID: {}", reg_id),
        };

        let data = register.data;
        register.data = data + value;
    }

    fn and_number_to_random_value(&self) {
        
    }

    fn display_spryte(&self) {
        
    }

    fn store_from_register_x_into_y(&self) {
        
    }

    fn call_address(&self) {
    }

    fn jump_to_machine_code(&self) {
        
    }

    fn set_register_i(&self) {
        
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
            9 => self.byte_skip_instruction(),
            9 => self.byte_skip_instruction(),
            10 => self.set_register_i(),
            11 => self.jump_offset_by_v0(),
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
