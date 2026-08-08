use crate::{memory::Memory, register::{Register, RegisterI}};

pub struct CPU {
    registers: [Register; 16],

    pc: u16, // Program Counter
    sp: u8,

    pub VF: Register, // NOTE: What is this register for?

    pub regI: RegisterI,

    pub dt_reg: Register,
    pub st_reg: Register,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            // WARN: Implement `Copy` for register
            registers: [
                Register::new(), // NOTE: 1
                Register::new(), // NOTE: 2
                Register::new(), // NOTE: 3
                Register::new(), // NOTE: 4
                Register::new(), // NOTE: 5
                Register::new(), // NOTE: 6
                Register::new(), // NOTE: 7
                Register::new(), // NOTE: 8
                Register::new(), // NOTE: 9
                Register::new(), // NOTE: 10
                Register::new(), // NOTE: 11
                Register::new(), // NOTE: 12
                Register::new(), // NOTE: 13
                Register::new(), // NOTE: 14
                Register::new(), // NOTE: 15
                Register::new()], // NOTE: 16
            VF: Register::new(),

            regI: RegisterI { data: 0 },

            dt_reg: Register::new(),
            st_reg: Register::new(),

            pc: 0, // idk what bytes the PC uses
            sp: 0, // Stack pointer
        }
    }

    pub fn get_register_data(&mut self, reg_id: usize) -> u8 {
            let len = self.registers.len();
            if reg_id > len || 0 > reg_id {
                panic!("Invalid register ID: {}", reg_id);
            }

            self.registers[reg_id].data
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
