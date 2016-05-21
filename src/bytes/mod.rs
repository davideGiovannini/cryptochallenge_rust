

pub fn xor(bytes_1: &[u8], bytes_2: &[u8]) -> Vec<u8> {
    assert_eq!(bytes_1.len(), bytes_2.len());

    let mut result = Vec::with_capacity(bytes_1.len());

    for (b1, b2) in bytes_1.iter().zip(bytes_2) {
        result.push(b1 ^ b2);
    }

    result
}







#[cfg(test)]
mod bytes_tests {
    use super::xor;

    #[test]
    fn test_xor() {
        assert_eq!(xor(&vec![0xF0, 0x0F], &vec![0x0F, 0x0F]), vec![0xFF, 0x00]);
    }
}
