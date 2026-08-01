pub fn print_decimal_in_16_bits_binary(number: &u8) {
    println!("{:16b}", number);
}

pub fn print_bytes_in_binary(numbers: &[u8]) {
    for &num in numbers {
        println!("{:08b}", num);
    }
}

pub fn print_bytes_in_hex(rom: &Vec<u8>) {
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 { println!(""); }
    }

    println!("");
}

pub fn print_bytes_vector(bytes: &Vec<u8>) {
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
