pub struct Register {
    pub data: u8
}

pub struct RegisterI {
    pub data: u16
}

impl Register {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

struct DelayTimerRegister {
    reg: Register
}

struct SoundTimerRegister {
    reg: Register
}
