

/// Encrypt a sequence of bytes using repeating XOR with the given Key.
pub fn rep_xor(bytes_stuff: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes_stuff.len());

    let mut key_iter = key.iter().cycle();

    for byte in bytes_stuff {
        result.push(byte ^ key_iter.next().unwrap());
    }

    result
}
