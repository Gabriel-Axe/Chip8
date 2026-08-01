
struct Register {
    data: u16
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
    V16: Register,

    VF: Register,

    regI: Register,

    pc: u16,
    stack_pointer: u16,

    dt_reg: Register,
    st_reg: Register,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
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
            V16: Register::new(),

            VF: Register::new(),

            regI: Register::new(),

            dt_reg: Register::new(),
            st_reg: Register::new(),

            pc: 0, // idk what bytes the PC uses
            stack_pointer: 0, // Stack pointer
        }
    }

    pub  fn hi() {
        
    }
}
