struct Chip8 {
    memory: Memory,
    reg_delay: DelayTimerRegister,
    reg_sound: SoundTimerRegister,
    pc: ProgramCounter,
    sp: StackPointer
}

impl Chip8 { fn new() -> Chip8 { let memory = Memory::new(); let reg_delay = DelayTimerRegister{current: 0};
        let reg_sound = SoundTimerRegister{current: 0};
        let pc = ProgramCounter{count: 0};
        let sp = StackPointer{address: 0};

        Chip8 {
            memory: memory,
            reg_delay: reg_delay,
            reg_sound: reg_sound,
            pc: pc,
            sp: sp
        }
    }
    fn clear_screen(self) {
        
    }

    fn return_from_subroutine(mut self) {
        self.sp.address -= 1;
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

    // fn somethingelse() {
    //     for byte in bytes  {
    //         instruction = 
    //             match byte {
    //                 224 => self.clear_screen(),           
    //                 238 => self.return_from_subroutine(),
    //                 _ => break
    //             }
    //     }
    }
