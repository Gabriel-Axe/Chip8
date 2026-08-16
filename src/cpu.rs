use crate::{debug_printer::DebugPrinter, memory::Memory, register::{Register, RegisterI}};

pub struct CPU {
    registers: [Register; 16],

    pc: u16, // Program Counter
    stack_pointer: u8,

    pub VF: Register, // NOTE: What is this register for?

    pub regI: RegisterI,

    pub dt_reg: Register,
    pub st_reg: Register,
}

impl CPU {
    pub fn new() -> Self {
        DebugPrinter::log_info("create CPU".to_string());
        CPU {
            // WARN: Implement `Copy` for register
            registers: [Register::new(); 16],
            VF: Register::new(),

            regI: RegisterI { data: 0 },

            dt_reg: Register::new(),
            st_reg: Register::new(),

            pc: 0x200,
            stack_pointer: 0,
        }
    }

    fn log_cpu_action(action: String) {
        DebugPrinter::log_action("cpu".to_string(), action);
    }

    pub fn get_register_data(&mut self, reg_id: usize) -> u8 {
            let len = self.registers.len();
            if reg_id > len || 0 > reg_id {
                panic!("Invalid register ID: {}", reg_id);
            }

            self.registers[reg_id].data
        }

    pub fn fetch_instruction_in_memory(&mut self, memory: &Memory) -> u8 {
        CPU::log_cpu_action("fetch instruction".to_string());
        let instruction = memory.fetch_in_address(self.pc, false);
        self.increment_pc();
        DebugPrinter::log_state(format!("instruction: 0x{:04X}", instruction));
        instruction
        // NOTE: Maybe i could refactor this into the CPU having
        // a internal instruction management and storing the
        // instruction instead of passing back to the Chip8
    }

    pub fn set_register_data(&mut self, reg_id: usize, val: u8) {
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
        self.stack_pointer += 1;
    }

    pub fn decrement_sp(&mut self) {
        self.stack_pointer -= 1;
    }

    pub fn get_sp(&mut self) -> u8 {
        self.stack_pointer
    }

    pub fn get_pc(&mut self) -> u8 {
        self.stack_pointer
    }

    pub fn increment_pc(&mut self) {
        // WARN: This should not be public
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
        let vy_data = self.registers[y as usize].data;
        let vx_data = self.registers[x as usize].data;

        if (vx_data + vy_data) > 255 {
            self.VF.data = 1;
        }

        let val = vx_data + vy_data;
        self.registers[x as usize].data = val;
    }

    // pub fn sub_values(&mut self, x: u8, y: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let vy = self.get_register_data(y);
    //     let mut vf = self.VF;
    //
    //     if vx.data > vy.data {
    //         vf.data = 1;
    //     }
    //
    //     let val = vy.data - vx.data;
    //     vx.data = val;
    // }

    // pub fn store_delay_timer_in_vx(&mut self, reg_id: u8) {
    //     let mut register = self.get_register_data(reg_id);
    //     register.data = self.dt_reg.data;
    // }

    // pub fn store_vx_in_delay_timer(&mut self, reg_id: u8) {
    //     let mut register = self.get_register_data(reg_id);
    //     self.dt_reg.data = register.data;
    // }

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

    // pub fn store_vx_in_sound_timer(&mut self, reg_id: u8) {
    //     let mut register = self.get_register_data(reg_id);
    //     self.st_reg.data = register.data;
    // }

    // pub fn subn_values(&mut self, x: u8, y: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let vy = self.get_register_data(y);
    //     let mut vf = self.VF;
    //
    //     if vx.data > vy.data {
    //         vf.data = 1;
    //     }
    //
    //     let val = vx.data - vy.data;
    //     vx.data = val;
    // }

    // pub fn shr_values(&mut self, x: u8) {
    //     let mut vx = self.get_register_data(x);
    //     let mut vf = self.VF;
    //
    //     if vx.data & 1 == 1 {
    //         vf.data = 1;
    //     } else {
    //         vf.data = 0;
    //     }
    //
    //     vx.data = vx.data / 2;
    // }

    // pub fn return_from_subroutine(&mut self, memory: Memory) {
    //     let addr = memory.stack[self.sp];
    //     self.set_program_counter_to_address(addr);
    //     self.decrement_sp();
    // }

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
