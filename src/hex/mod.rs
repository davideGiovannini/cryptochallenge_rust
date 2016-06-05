

/// Converts a given byte sequence to a string representation in hexadecimal.
pub fn to_hex_str(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(2 * bytes.len());

    for byte in bytes {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}


/// Converts a given hex string to the corresponding sequence of bytes.
pub fn parse_hex_str(bytes: &[u8]) -> Vec<u8> {
    assert!(bytes.len() % 2 == 0);

    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() / 2);
    let mut val: u8;

    for mut i in 0..(bytes.len() / 2) {
        i *= 2;
        val = parse(bytes[i] as char).unwrap() * 16 + parse(bytes[i + 1] as char).unwrap();

        result.push(val);
    }
    result
}


fn parse(ch: char) -> Option<u8> {
    Some(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'C' | 'c' => 12,
        'D' | 'd' => 13,
        'E' | 'e' => 14,
        'F' | 'f' => 15,
        _ => {
            println!("Invalid character: {}", ch);
            return None;
        }
    })
}


#[cfg(test)]
mod hex_tests {
    use super::parse_hex_str;
    use super::to_hex_str;

    #[test]
    fn test_empty() {
        assert_eq!(parse_hex_str(b""), b"")
    }

    #[test]
    fn test_TfS() {
        assert_eq!(parse_hex_str(b"546653"), b"TfS");
    }

    #[test]
    fn test_crypto() {
        assert_eq!(parse_hex_str(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
                   "I'm killing your brain like a poisonous mushroom".as_bytes())
    }

    #[test]
    fn test_natale() {
        assert_eq!(parse_hex_str(b"626162626F206E6174616C6520652720756E206964696F7461"),
                   b"babbo natale e' un idiota");
    }


    #[test]
    fn test_to_str() {
        let string = "626162626f206e6174616c6520652720756e206964696f7461";
        assert_eq!(to_hex_str(&parse_hex_str(string.as_bytes())), string);
    }
}
