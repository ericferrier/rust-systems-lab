pub fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());

    for (i, byte) in data.iter().enumerate() {
        let key_byte = key[i % key.len()];
        result.push(byte ^ key_byte);
    }

    result
}