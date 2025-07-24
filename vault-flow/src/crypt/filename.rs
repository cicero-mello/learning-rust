const CHARSET: &[u8; 64] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_";

fn byte_to_bits(byte: u8) -> Vec<u8> {
    return (0..=7).rev().map(|i| (byte >> i) & 1).collect();
}

fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    return bytes.iter().flat_map(|&byte| byte_to_bits(byte)).collect();
}

fn bits_to_u8(bits: &[u8]) -> u8 {
    return bits.iter().fold(0, |acc, &bit| (acc << 1) | bit);
}

fn u6_to_bits(value: u8) -> Vec<u8> {
    return (0..6).rev().map(|i| (value >> i) & 1).collect();
}

fn base64_str_to_bits(encoded: &str) -> Vec<u8> {
    return encoded.chars().flat_map(|ch| {
        let char_value = CHARSET.iter().position(|&c| c as char == ch).unwrap() as u8;
        return u6_to_bits(char_value);
    }).collect();
}

pub fn encode(file_name: &str) -> String {
    let bytes_vector: Vec<u8> = file_name.as_bytes().to_vec();
    let mut bits_vector: Vec<u8> = bytes_to_bits(&bytes_vector);

    let mut result = String::new();

    loop {
        if bits_vector.len() <= 6 {
            while bits_vector.len() != 6 {
                bits_vector.push(0);
            }

            let charset_index = bits_to_u8(&bits_vector);
            result.push(CHARSET[charset_index as usize] as char);
            break;
        }

        let chunk: Vec<u8> = bits_vector.drain(0..6).collect();
        let charset_index = bits_to_u8(&chunk);
        result.push(CHARSET[charset_index as usize] as char);
    }

    return result;
}

pub fn decode(encoded: &str) -> String {
    let bits_vector: Vec<u8> = base64_str_to_bits(encoded);

    let mut result_bytes: Vec<u8> = Vec::new();

    let mut bit_index = 0;

    while bit_index + 8 <= bits_vector.len() {
        let chunk_8_bits  = &bits_vector[bit_index..bit_index+8];
        result_bytes.push(bits_to_u8(chunk_8_bits));
        bit_index += 8;
    }

    return String::from_utf8(result_bytes).expect("Error");
}
