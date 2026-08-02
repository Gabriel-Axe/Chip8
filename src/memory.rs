
pub struct Register {
    pub data: u8
}

pub struct RegisterI {
    pub data: u16
}

impl Register {
    fn new() -> Self {
        Self { data: 0 }
    }
}

struct DelayTimerRegister {
    reg: Register
}

struct SoundTimerRegister {
    reg: Register
}

pub struct Memory {
    pub V0: Register,
    pub V1: Register,
    pub V2: Register,
    pub V3: Register,
    pub V4: Register,
    pub V5: Register,
    pub V6: Register,
    pub V7: Register,
    pub V8: Register,
    pub V9: Register,
    pub V10: Register,
    pub V11: Register,
    pub V12: Register,
    pub V13: Register,
    pub V14: Register,
    pub V15: Register,

    pub VF: Register,

    pub regI: RegisterI,

    pub pc: u16,
    pub stack_pointer: u16,

    pub dt_reg: Register,
    pub st_reg: Register,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
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
            stack_pointer: 0, // Stack pointer
        }
    }
}
