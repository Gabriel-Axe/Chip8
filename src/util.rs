fn u16_to_decimal(numbers: Vec<u16>) {
    for num in numbers {
        println!("{:016b}", num);
    }
}

fn decimal_to_binary_one_number(number: &u8) {
    println!("{:08b}", number);
}

fn decimal_to_binary_only(numbers: &[u8]) {
    for &num in numbers {
        println!("{:08b}", num);
    }
}

fn read_rom_as_hex(rom: &Vec<u8>) {
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 { println!(""); }
    }

    println!("");
}

fn read_byte_by_byte_from_file(bytes: &Vec<u8>) {
    for byte in bytes {
        println!("{}", byte);
    }
}

// fn convert_8bits_to_16bits(bytes: &Vec<u8>) -> Vec<u16> {
fn convert_u8_to_u16(bytes: &[u8]) -> Result<Vec<u16>, String> {
    // if bytes.len() % 2 != 0 {
    //     let res = String::from_str("Cannot convert u8 vector, there is a odd number of values");
    //     let mes = res.expect("Could not unwrap :(");
    //     return Err(mes);
    // }

    let words: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    return Ok(words)
}
