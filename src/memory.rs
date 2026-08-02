pub struct Memory {
    pub data: [u8; 3583],
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 3583],
            stack: [0; 16],
        }
    }
}
