pub struct Memory {
    data: [u16; 4096],
    pub stack: [u16; 16],
}

const ADDR_RESERVED_END: u16 = 0x200;

impl Memory {
    pub fn new() -> Self {
        log::info!("Creating Memory");
        Memory {
            data: [0; 4096],
            stack: [0; 16],
        }
    }

    pub fn log_contents(&self) {
        for i in 0..self.size() {
            let address = i as u16 + ADDR_RESERVED_END;
            let data = self.fetch_in_address(address);
            log::debug!("Address: {:04X} | Contents {:04X}", i as u16 + address, data);
        }
    }

    fn access_address(&self, addr: u16) -> &[u16] {
        let max_len = self.data.len();
        let address = addr + ADDR_RESERVED_END;
        if address as usize > max_len {
            panic!("Tried to fetch memory outside of limits (address {})", addr);
        }

        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len() - ADDR_RESERVED_END as usize
    }

    pub fn fetch_in_address(&self, addr: u16) -> u16 {
        self.access_address(addr)[addr as usize]
    }

    pub fn set_in_address(&mut self, addr: u16, value: u16) {
        let address = addr + ADDR_RESERVED_END;
        log::debug!("Storing {:04X} in address {:04X}", value, address);
        // WARN: Maybe I shouldnt rely on a method to not panic
        // and then set the value, maybe I should swet the value
        // directly
        self.access_address(address);
        self.data[(address) as usize] = value;
        let stored = self.fetch_in_address(address);
        log::debug!("Stored in {:04X}: {:04X}", address, stored);
    }
}
