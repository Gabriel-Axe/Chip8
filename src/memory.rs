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
        let boundary: usize = self.size();
        for address in 0..boundary {
            let data = self.get_data_in_address(address);
            DebugPrinter::log_state(format!("address: 0x{:04X} | contents 0x{:04X}", address as u16, data));
        }
    }

    fn can_access_address(&self, addr: usize) -> bool {
        let max_len = self.data.len();
        Memory::log_memory_action(format!("compare addr ({}) to max length ({})", addr, max_len));
        (addr as usize) <= max_len
    }

    pub fn get_data_in_address(&self, addr: usize) -> &u8 {
        if !self.can_access_address(addr) {
            panic!("Tried to fetch memory in address {}, which is outside of limits ({})", addr, self.data.len());
            // WARN: Possible space for logging a error
        }

        let contents = &self.data[addr as usize];
        Memory::log_memory_action(format!("fetch in address 0x{:04X}: 0x{:04X}", addr, contents));

        &contents
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    fn log_memory_action(action: String) {
        DebugPrinter::log_action("memory".to_string(), action);
    }

    // pub fn offset_memory_address_access(&self, addr: u16) -> usize {
    //     let address = (addr + PROGRAM_START_OFFSET) as usize;
    //     Memory::log_memory_action(format!("offset memory address. before: 0x{:04X}, after: 0x{:04X}", addr, address));
    //     address
    // }

    pub fn set_value_in_address(&mut self, addr: usize, value: u8) {
        Memory::log_memory_action(format!("store 0x{:04X} in address 0x{:04X}", value, addr));

        if !self.can_access_address(addr) { return }
        self.data[(addr) as usize] = value;

        let stored = self.get_data_in_address(addr);
        DebugPrinter::log_state(format!("memory address: 0x{:04X}, value stored: 0x{:02X}", addr, stored));
    }
}
