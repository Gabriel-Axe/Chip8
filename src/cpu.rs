use crate::{memory::Memory, register::{Register, RegisterI}};

pub struct CPU {
    V0: Register,
    V1: Register,
    V2: Register,
    V3: Register,
    V4: Register,
    V5: Register,
    V6: Register,
    V7: Register,
    V8: Register,
    V9: Register,
    V10: Register,
    V11: Register,
    V12: Register,
    V13: Register,
    V14: Register,
    V15: Register,

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
            V0: Register::new(),
            V1: Register::new(),
            V2: Register::new(),
            V3: Register::new(),
            V4: Register::new(),
            V5: Register::new(),
            V6: Register::new(),
            V7: Register::new(),
            V8: Register::new(),
            V9: Register::new(),
            V10: Register::new(),
            V11: Register::new(),
            V12: Register::new(),
            V13: Register::new(),
            V14: Register::new(),
            V15: Register::new(),

            VF: Register::new(),

            regI: RegisterI { data: 0 },

            dt_reg: Register::new(),
            st_reg: Register::new(),

            pc: 0, // idk what bytes the PC uses
            sp: 0, // Stack pointer
        }
    }

    pub fn get_vx_register_by_id(&mut self, reg_id: u8) -> &mut Register {
        match reg_id {
            0 => &mut self.V0,
            1 => &mut self.V1,
            2 => &mut self.V2,
            3 => &mut self.V3,
            4 => &mut self.V4,
            5 => &mut self.V5,
            6 => &mut self.V6,
            7 => &mut self.V7,
            8 => &mut self.V8,
            9 => &mut self.V9,
            10 => &mut self.V10,
            11 => &mut self.V11,
            13 => &mut self.V13,
            14 => &mut self.V14,
            12 => &mut self.V12,
            15 => &mut self.V15,
            _ => panic!("Invalid register ID: {}", reg_id),
        }
    }

    pub fn set_program_counter_to_address(&mut self, address: u16) {
        self.pc = address;
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
        let mut vx = self.get_vx_register_by_id(x);
        let vy = self.get_vx_register_by_id(y);
        let mut vf = self.VF;

        if (vx.data + vy.data) > 255 {
            vf.data = 1;
        }

        let val = vx.data + vy.data;
        vx.data = val;
    }

    pub fn sub_values(&mut self, x: u8, y: u8) {
        let mut vx = self.get_vx_register_by_id(x);
        let vy = self.get_vx_register_by_id(y);
        let mut vf = self.VF;

        if vx.data > vy.data {
            vf.data = 1;
        }

        let val = vy.data - vx.data;
        vx.data = val;
    }

    pub fn store_delay_timer_in_vx(&mut self, reg_id: u8) {
        let mut register = self.get_vx_register_by_id(reg_id);
        register.data = self.dt_reg.data;
    }

    pub fn store_vx_in_delay_timer(&mut self, reg_id: u8) {
        let mut register = self.get_vx_register_by_id(reg_id);
        self.dt_reg.data = register.data;
    }

    pub fn add_vx_to_register_i(&mut self, reg_id: u8) {
        let mut register = self.get_vx_register_by_id(reg_id);
        let i_val = self.regI.data;
        self.regI.data = i_val + register.data;
    }

    pub fn ld_store_in_memory(&mut self, memory: Memory) {
        let cur_addr = self.regI.data;
        for id_r in 0..15 {
            let reg = self.get_vx_register_by_id(id_r);
            memory.data[cur_addr] = reg.data;
            cur_addr += 1;
        }
    }

    pub fn ld_read_in_memory(&mut self, memory: Memory) {
        let cur_addr = self.regI.data;
        for id_r in 0..15 {
            let reg = self.get_vx_register_by_id(id_r);
            reg.data = memory.data[cur_addr];
        }
    }

    pub fn store_vx_in_sound_timer(&mut self, reg_id: u8) {
        let mut register = self.get_vx_register_by_id(reg_id);
        self.st_reg.data = register.data;
    }

    pub fn subn_values(&mut self, x: u8, y: u8) {
        let mut vx = self.get_vx_register_by_id(x);
        let vy = self.get_vx_register_by_id(y);
        let mut vf = self.VF;

        if vx.data > vy.data {
            vf.data = 1;
        }

        let val = vx.data - vy.data;
        vx.data = val;
    }

    pub fn shr_values(&mut self, x: u8) {
        let mut vx = self.get_vx_register_by_id(x);
        let mut vf = self.VF;

        if vx.data & 1 == 1 {
            vf.data = 1;
        } else {
            vf.data = 0;
        }

        vx.data = vx.data / 2;
    }

    pub fn return_from_subroutine(&mut self, memory: Memory) {
        let addr = memory.stack[self.sp];
        self.set_program_counter_to_address(addr);
        self.decrement_sp();
    }

    pub fn shl_values(&mut self, x: u8) {
        let mut vx = self.get_vx_register_by_id(x);
        let mut vf = self.VF;

        if vx.data & 1 == 1 {
            vf.data = 1;
        } else {
            vf.data = 0;
        }

        vx.data = vx.data * 2;
    }

    fn nand(&self, x: u8, y: u8) -> u8 {
        !(x & y)
    }
}
