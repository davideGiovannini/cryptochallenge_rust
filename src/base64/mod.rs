
pub fn to_base64(string: &str) -> String {
    let bytes = string.as_bytes();
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

const BASE64_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                  'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                                  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                                  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];


#[cfg(test)]
mod base64_tests {
    use super::to_base64;

    #[test]
    fn test_empty() {
        assert_eq!(to_base64(&""), "")
    }

    #[test]
    fn test_Man() {
        assert_eq!(to_base64(&"Man"), "TWFu")
    }

    #[test]
    fn test_M() {
        assert_eq!(to_base64(&"M"), "TQ==")
    }

    #[test]
    fn test_Horse() {
        assert_eq!(to_base64(&"Horse"), "SG9yc2U=")
    }
    #[test]
    fn test_hurricane() {
        assert_eq!(to_base64(&"hurricane"), "aHVycmljYW5l")
    }

    #[test]
    fn test_banana() {
        assert_eq!(to_base64(&"banana"), "YmFuYW5h")
    }

    #[test]
    fn test_banana_nababa() {
        assert_eq!(to_base64(&"banana nababa"), "YmFuYW5hIG5hYmFiYQ==")
    }

    #[test]
    fn test_trololol() {
        assert_eq!(to_base64(&"trololol"), "dHJvbG9sb2w=")
    }
}
