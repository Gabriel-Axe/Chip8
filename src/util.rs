pub fn print_bytes_in_16_binary(bytes: &Vec<u16>) {
    for &byte in bytes {
        println!("{:016b}", byte);
    }
}

pub fn get_4th_nibble(bytes: &Vec<u16>) -> Vec<u16> {
    let mut nvec: Vec<u16> = Vec::new();
    for &byte in bytes {
        let nbyte = byte & 61440; // NOTE: equivalent to 1111 0000 0000 0000
        nvec.push(nbyte);
    }

    return nvec;
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

pub fn convert_u8_to_u16(bytes: &[u8]) -> Result<Vec<u16>, String> {
    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    return Ok(words)
}
