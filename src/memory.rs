use crate::debug_printer::DebugPrinter;

pub struct Memory {
    data: [u8; 4096],
    pub stack: [u8; 16],
}

pub const PROGRAM_START_OFFSET: u16 = 0x200;
// apparently i dont need this

impl Memory {
    pub fn new() -> Self {
        DebugPrinter::log_info("Creating Memory".to_string());
        Memory {
            data: [0; 4096],
            stack: [0; 16],
        }
    }

    pub fn log_contents(&self) {
        // DebugPrinter::log_state(format!("memory size: {}", self.size()));
        let boundary: usize = self.size();
        for address in 0..boundary {
            let data = self.fetch_in_address(address as u16, false);
            DebugPrinter::log_state(format!("address: 0x{:04X} | contents 0x{:04X}", address as u16, data));
        }
    }

    fn access_address(&self, addr: u16) -> &[u8] {
        let max_len = self.data.len();
        if addr as usize >= max_len {
            panic!("Tried to fetch memory outside of limits (address {})", addr);
        }

        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    fn log_memory_action(action: String) {
        DebugPrinter::log_action("memory".to_string(), action);
    }

    pub fn offset_memory_address_access(&self, addr: u16) -> usize {
        let address = (addr + PROGRAM_START_OFFSET) as usize;
        Memory::log_memory_action(format!("offset memory address. before: 0x{:04X}, after: 0x{:04X}", addr, address));
        address
    }

    pub fn fetch_in_address(&self, addr: u16, offset: bool) -> u8 {
        // let addr = self.offset_memory_address_access(addr);
        // if offset == true {
        //     log::debug!("Offset is set to: {:}", offset);
        //     let addr = self.offset_memory_address_access(addr);
        // }
        //
        // // WARN: I removed the jump offset here, because of... JMP
        // let address = addr as usize;
        let contents = self.access_address(addr as u16)[addr as usize];
        Memory::log_memory_action(format!("fetch in address 0x{:04X}: 0x{:04X}", addr, contents));
        contents
    }

    pub fn set_in_address(&mut self, addr: u16, value: u8) {
        let address = self.offset_memory_address_access(addr);
        Memory::log_memory_action(format!("store 0x{:04X} in address 0x{:04X}", value, address));
        // WARN: Maybe I shouldnt rely on a method to not panic
        // and then set the value, maybe I should swet the value
        // directly
        self.access_address(address as u16);
        self.data[(address) as usize] = value;
        // WARN: In the function bellow, i use ADDR_RESERVED_END, however,
        // fetch_in_address already uses the offset to fetch the data
        // TODO: Find a way to fix this
        let stored = self.fetch_in_address(addr, false);
        DebugPrinter::log_state(format!("address: 0x{:04X} stored: 0x{:04X} parameter: 0x{:04X}", address, stored, value));
    }
}
