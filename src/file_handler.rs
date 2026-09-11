use std::{fs, path::PathBuf};

use crate::{load_rom, memory::{self, Memory, PROGRAM_START_OFFSET}, util::mirror_bits};

pub struct RomHandler
{
    rom: Vec<u8>,
    path: Option<String>
}

impl RomHandler {
    
    pub fn new(file: &str, path: Option<&str>) -> Self {
        // Chip8::log_chip8_action("Loading ROM".to_string());

        let existent_path = match path {
            Some(p) => p,
            None => ".",
        }.to_string();

        let mut path_buf = PathBuf::from(existent_path.clone());
        // path_buf.push(file);
        let file = match fs::read(path_buf.clone()) {
          Ok(f) => f,
          Err(e) => 
          {
              let path_err = match path_buf.to_str() {
                  Some(p) => p,
                  None => "null", // WARN: What to do in here?
              };
              panic!("Could not load ROM: {} in path {}, reason: {}", file, path_err, e);
          }
        };

        Self { 
            rom: file, 
            path: Some(existent_path),
        }
    }

    pub fn set_folder(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn get_rom(&self) -> Vec<u8> {
        self.rom.clone()
    }
    
    pub fn load_rom(&self, memory: &mut Memory) {

        for (mut addr, instruction) in 
                self
                .get_rom() 
                .iter() 
                .enumerate() {
            // let instruction = mirror_bits(*instruction);
            memory.set_value_in_address(addr + PROGRAM_START_OFFSET as usize, *instruction);
        }

        // DebugPrinter::log_info(format!("loaded ROM: {}", filename));
    }
}
