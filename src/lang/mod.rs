
use bytes::xor_byte;
use std::char;

pub fn crack_xor_cypher(bytes: &[u8]) -> (u8, char, String, f64) {
    let mut most_likely = (0.0, 0);

    for key in 0..256u16 {
        let result = &xor_byte(bytes, key as u8);
        let score = score_as_english(result);
        if score > most_likely.0 {
            most_likely = (score, key as u8);
        }
    }

    return (most_likely.1,
            char::from_u32(most_likely.1 as u32).unwrap_or(' '),
            String::from_utf8(xor_byte(bytes, most_likely.1))
        .unwrap_or("Error while converting from bytes to utf8 string".to_string()),
            most_likely.0);
}


pub fn score_as_english(bytes: &[u8]) -> f64 {
    let mut value = 1.0;
    for byte in bytes {
        value *= frequency_of(char::from_u32(*byte as u32).unwrap_or(' '))
    }
    value
}


fn frequency_of(c: char) -> f64 {
    match c {
        'E' | 'e' => 12.02,
        'T' | 't' => 9.10,
        'A' | 'a' => 8.12,
        'O' | 'o' => 7.68,
        'I' | 'i' => 7.31,
        'N' | 'n' => 6.95,
        'S' | 's' => 6.28,
        'R' | 'r' => 6.02,
        'H' | 'h' => 5.92,
        'D' | 'd' => 4.32,
        'L' | 'l' => 3.98,
        'U' | 'u' => 2.88,
        'C' | 'c' => 2.71,
        'M' | 'm' => 2.61,
        'F' | 'f' => 2.30,
        'Y' | 'y' => 2.11,
        'W' | 'w' => 2.09,
        'G' | 'g' => 2.03,
        'P' | 'p' => 1.82,
        'B' | 'b' => 1.49,
        'V' | 'v' => 1.11,
        'K' | 'k' => 0.69,
        'X' | 'x' => 0.17,
        'Q' | 'q' => 0.11,
        'J' | 'j' => 0.10,
        'Z' | 'z' => 0.07,
        _ => 1.0,
    }
}
