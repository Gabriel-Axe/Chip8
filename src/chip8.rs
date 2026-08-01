use crate::{memory::Memory, util::get_instruction_nibble};

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

    fn byte_skip_instruction(&self) {
        
    }

    fn set_value_at_register(&self) {
        
    }

    fn add_byte_operation(&self) {
        
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

    fn read_instruction(&self, mut instruction: u16) {
        let inst_type = get_instruction_nibble(instruction);

        let nibble_3 = instruction >> 3;
        let nibble_2 = instruction >> 2;
        let nibble_1 = instruction >> 1;

        match inst_type {
            0 => self.clear_screen(),
            1 => self.jump_to_address(),
            2 => self.call_address(),
            3 => self.skip_instruction_if(),
            4 => self.byte_skip_instruction(),
            5 => self.skip_instruction_if(),
            6 => self.set_value_at_register(),
            7 => self.add_byte_operation(),
            9 => self.byte_skip_instruction(),
            9 => self.byte_skip_instruction(),
            10 => self.set_register_i(),
            11 => self.jump_offset_by_v0(),
            12 => self.and_number_to_random_value(),
            13 => self.display_spryte(),
            _ => return
        }
    }

    pub fn interpret(&self, instructions: &Vec<u16>) {
        for &inst in instructions {
            self.read_instruction(inst);
        }
    }
}
