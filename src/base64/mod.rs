
/// Encodes in base64 a given byte sequence.
pub fn to_base64(bytes: &[u8]) -> String {
    let mut result = String::with_capacity((4 * (bytes.len() / 3)));

    let (mut remainder, mut rem_bytes) = (0u8, 0u8);
    let mut value: usize;

    for byte in bytes {
        value = ((remainder | (byte >> rem_bytes)) >> 2) as usize;
        result.push(BASE64_TABLE[value]);

        match rem_bytes {
            2 => {
                remainder = byte << 4;
                rem_bytes = 4;
            }
            4 => {
                value = (byte & 0b00111111) as usize;
                result.push(BASE64_TABLE[value]);
                rem_bytes = 0;
                remainder = 0;
            }
            _ => {
                remainder = byte << 6;
                rem_bytes = 2;
            }
        }
    }

    if rem_bytes != 0 {
        value = (remainder >> 2) as usize;
        result.push(BASE64_TABLE[value]);
        result.push('=');
        if rem_bytes == 2 {
            result.push('=')
        }
    }

    return result;
}

/// Decodes a base64 string into a byte sequence.
pub fn from_base64(data: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity((3 * (data.len() / 4)));
    println!("{}", result.capacity());
    let mut decoded: u8;
    let mut bytes_needed = 0usize;

    for character in data.chars() {
        if character != '=' {
            decoded = decode_char(&character);
            match bytes_needed {
                0 => {
                    result.push(decoded << 2);
                    bytes_needed = 2;
                }
                2 => {
                    *result.last_mut().unwrap() |= decoded >> 4;
                    result.push(decoded << 4);
                    bytes_needed = 4;
                }
                4 => {
                    *result.last_mut().unwrap() |= decoded >> 2;
                    result.push(decoded << 6);
                    bytes_needed = 6;
                }
                6 => {
                    *result.last_mut().unwrap() |= decoded;
                    bytes_needed = 0;
                }
                _ => unreachable!("Error in the algorithm!"),
            }
        } else {
            break;
        }
    }

    if bytes_needed == 4 || bytes_needed == 6 {
        // Discard last empty byte
        result.pop();
    }

    result
}




const BASE64_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                  'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                                  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                                  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

fn decode_char(c: &char) -> u8 {
    match *c {
        'A' => 0u8,
        'B' => 1u8,
        'C' => 2u8,
        'D' => 3u8,
        'E' => 4u8,
        'F' => 5u8,
        'G' => 6u8,
        'H' => 7u8,
        'I' => 8u8,
        'J' => 9u8,
        'K' => 10u8,
        'L' => 11u8,
        'M' => 12u8,

        'N' => 13u8,
        'O' => 14u8,
        'P' => 15u8,
        'Q' => 16u8,
        'R' => 17u8,
        'S' => 18u8,
        'T' => 19u8,
        'U' => 20u8,
        'V' => 21u8,
        'W' => 22u8,
        'X' => 23u8,
        'Y' => 24u8,
        'Z' => 25u8,

        'a' => 26u8,
        'b' => 27u8,
        'c' => 28u8,
        'd' => 29u8,
        'e' => 30u8,
        'f' => 31u8,
        'g' => 32u8,
        'h' => 33u8,
        'i' => 34u8,
        'j' => 35u8,
        'k' => 36u8,
        'l' => 37u8,
        'm' => 38u8,

        'n' => 39u8,
        'o' => 40u8,
        'p' => 41u8,
        'q' => 42u8,
        'r' => 43u8,
        's' => 44u8,
        't' => 45u8,
        'u' => 46u8,
        'v' => 47u8,
        'w' => 48u8,
        'x' => 49u8,
        'y' => 50u8,
        'z' => 51u8,

        '0' => 52u8,
        '1' => 53u8,
        '2' => 54u8,
        '3' => 55u8,
        '4' => 56u8,
        '5' => 57u8,
        '6' => 58u8,
        '7' => 59u8,
        '8' => 60u8,
        '9' => 61u8,
        '+' => 62u8,
        '/' => 63u8,
        _ => panic!("Wrong character for a base64 string"),
    }
}





#[cfg(test)]
mod base64_tests {
    // use super::to_base64;
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(to_base64(&"".as_bytes()), "")
    }

    #[test]
    fn test_Man() {
        assert_eq!(to_base64(&"Man".as_bytes()), "TWFu")
    }

    #[test]
    fn test_M() {
        assert_eq!(to_base64(&"M".as_bytes()), "TQ==")
    }

    #[test]
    fn test_Horse() {
        assert_eq!(to_base64(&"Horse".as_bytes()), "SG9yc2U=")
    }
    #[test]
    fn test_hurricane() {
        assert_eq!(to_base64(&"hurricane".as_bytes()), "aHVycmljYW5l")
    }

    #[test]
    fn test_banana() {
        assert_eq!(to_base64(&"banana".as_bytes()), "YmFuYW5h")
    }

    #[test]
    fn test_banana_nababa() {
        assert_eq!(to_base64(&"banana nababa".as_bytes()),
                   "YmFuYW5hIG5hYmFiYQ==")
    }

    #[test]
    fn test_trololol() {
        assert_eq!(to_base64(&"trololol".as_bytes()), "dHJvbG9sb2w=")
    }


    #[test]
    fn test_decode() {
        let strings = vec!["Test value",
                           "",
                           "hurricane",
                           "Horse",
                           "M",
                           "Man",
                           "trololol",
                           "banana",
                           "banana nababa"];
        for string in strings {
            assert_eq!(from_base64(&to_base64(string.as_bytes())),
                       string.as_bytes())
        }
    }
}
