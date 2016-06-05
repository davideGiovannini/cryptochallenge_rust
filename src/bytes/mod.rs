
/// Computes the bitwise XOR between two byte sequences of the same lenght.
pub fn xor_seqs(bytes_1: &[u8], bytes_2: &[u8]) -> Vec<u8> {
    assert_eq!(bytes_1.len(), bytes_2.len());

    let mut result = Vec::with_capacity(bytes_1.len());

    for (b1, b2) in bytes_1.iter().zip(bytes_2) {
        result.push(b1 ^ b2);
    }

    result
}

/// Computes the bitwise XOR between each byte of a byte sequence and a given byte.
pub fn xor_byte(bytes_seq: &[u8], byte: u8) -> Vec<u8> {

    let mut result = Vec::with_capacity(bytes_seq.len());

    for b in bytes_seq {
        result.push(b ^ byte);
    }

    result
}

/// Computes the Hamming distance between two byte sequences of the same lenght.
///
/// The Hamming distance in this case is just the number of differing bits.
pub fn hamming_bit_distance(bytesA: &[u8], bytesB: &[u8]) -> usize {
    assert_eq!(bytesA.len(), bytesB.len());
    let mut distance = 0;

    for (byteA, byteB) in bytesA.iter().zip(bytesB) {
        if byteA != byteB {
            for pos in 0..8 {
                if byteA & (1 << pos) != byteB & (1 << pos) {
                    distance += 1;
                }
            }
        }
    }

    distance
}

#[cfg(test)]
mod bytes_tests {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(xor_seqs(&vec![0xF0, 0x0F], &vec![0x0F, 0x0F]),
                   vec![0xFF, 0x00]);
    }

    #[test]
    fn test_hamming() {
        assert_eq!(hamming_bit_distance(b"this is a test", b"wokka wokka!!!"),
                   37);
    }
}
