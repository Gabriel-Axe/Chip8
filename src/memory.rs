pub struct Memory {
    pub data: [u8; 3583],
    pub stack: [u16; 16],

    pub pc: u16,
    pub sp: u8,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 3583],
            stack: [0; 16],
            pc: 0, // idk what bytes the PC uses
            sp: 0, // Stack pointer
        }
    }
}
