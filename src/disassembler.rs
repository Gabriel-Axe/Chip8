use std::fs;

pub struct Disassembler {
    instructions: Vec<u8>,
}
impl Disassembler {

    pub fn read_instructions(&mut self, rom: Vec<u8>) {
        for instr in rom {
            self.instructions.push(instr);
        }
    }

    pub fn output_to_file() -> std::io::Result<()> {
        fs::write("hello", "hi")?;
        let bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03];
        fs::write("data.bin", bytes)?;
        Ok(())
        // for instr in self.instructions {
        // }
    }
}
