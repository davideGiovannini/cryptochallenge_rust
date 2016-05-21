use std::char;


pub fn hex_to_string(string: &str) -> String {
    let bytes = string.as_bytes();

    assert!(bytes.len() % 2 == 0);

    let mut result = String::with_capacity(bytes.len() / 2);
    let mut val: u32;

    for mut i in 0..(bytes.len() / 2) {
        i *= 2;
        val = parse(bytes[i] as char).unwrap() * 16 + parse(bytes[i + 1] as char).unwrap();

        result.push(char::from_u32(val).unwrap());
    }

    result
}


fn parse(ch: char) -> Option<u32> {
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
    use super::hex_to_string;

    #[test]
    fn test_empty() {
        assert_eq!(hex_to_string(""), "")
    }

    #[test]
    fn test_TfS() {
        assert_eq!(hex_to_string("546653"), "TfS");
    }

    #[test]
    fn test_crypto() {
        assert_eq!(hex_to_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
                   "I'm killing your brain like a poisonous mushroom")
    }

    #[test]
    fn test_natale() {
        assert_eq!(hex_to_string("626162626F206E6174616C6520652720756E206964696F7461"),
                   "babbo natale e' un idiota");
    }
}
