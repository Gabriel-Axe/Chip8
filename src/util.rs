pub fn print_bytes_in_16_binary(bytes: &Vec<u16>) {
    for &byte in bytes {
        println!("{:016b}", byte);
    }
}


pub fn print_bytes_in_binary(bytes: &Vec<u8>) {
    for &byte in bytes {
        println!("{:08b}", byte);
    }
}

pub fn print_bytes_in_hex(rom: &Vec<u8>) {
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 { println!(""); }
    }

    println!("");
}

pub fn print_bytes_in_deciaml(bytes: &Vec<u8>) {
    for byte in bytes {
        println!("{}", byte);
    }
}

pub fn convert_bytes_u8_to_u16(bytes: &[u8]) -> Result<Vec<u16>, String> {
    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    return Ok(words)
}

pub fn join_2_nibbles_into_u8(nibble_1: u8, nibble_2: u8) -> u8 {
    nibble_1 + nibble_2
}

pub fn join_3_nibbles_into_u8(nibble_1: u8, nibble_2: u8, nibble_3: u8) -> u8 {
    nibble_1 + nibble_2 + nibble_3
}

pub fn create_u32_vec_from_bool(bool_vec: Vec<bool>) -> Vec<u32> {
    let mut integer_vec: Vec<u32> = vec![0; bool_vec.len()];
    for val in 0..bool_vec.len() {
        let cur_val = bool_vec[val];
        if cur_val != false {
            integer_vec[val] = from_u8_rgb(255, 255, 255);
        }
    }

    return integer_vec;
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn from_u8_to_bin_color(val: u8) -> u32 {
    let (r, g, b) = (val as u32, val as u32, val as u32);
    (r << 16) | (g << 8) | b
}
