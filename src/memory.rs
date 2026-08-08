pub struct Memory {
    pub data: [u8; 4096],
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 4096],
            stack: [0; 16],
        }
    }
}
