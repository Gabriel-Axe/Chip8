use crate::register::{Register, RegisterI};

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
}
