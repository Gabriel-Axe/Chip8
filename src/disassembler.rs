use std::fs;

pub struct Disassembler {
    // instructions: Vec<u8>,
}
impl Disassembler {

    pub fn read_instructions(rom: Vec<u8>) {
        let mut instructions: Vec<u8> = Vec::new();
        for instr in rom {
            instructions.push(instr);
        }
    }

    pub fn output_to_file(binary_contents: Vec<u8>) -> std::io::Result<()> {
        let contents = match str::from_utf8(&binary_contents) {
            Ok(v) => v,
            Err(e) => panic!("Something went wrong: {}", e),
        };
        fs::write("rom.dasm", contents)?;
        Ok(())
    }
}
