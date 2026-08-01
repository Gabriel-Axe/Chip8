use crate::memory::{Memory};

struct Chip8 {
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 { 
        Chip8 {
            memory: Memory::new()
        }
    }

    fn clear_screen(self) {
        
    }

    fn return_from_subroutine(mut self) {
    }

    fn jump_to_address(self) {
        
    }

    fn jump_to_machine_code(self) {
        
    }

    fn read_instruction(self, mut instruction: Instruction) {
    }

    fn interpret(self, mut bytes: Vec<u8>) {
        let instruction_type = bytes.pop().unwrap(); // WARN: DANGER, unsafe operation
        let instruction_data = bytes.pop().unwrap();
        let instruction = Instruction {inst_type: instruction_type, inst_data: instruction_data, };
        self.read_instruction(instruction);
    }
