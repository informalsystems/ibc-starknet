pub fn u64_array_to_u8_array<const N: usize, const M: usize>(arr: &[u64; N]) -> [u8; M] {
    let mut result = [0u8; M];
    for (i, &n) in arr.iter().enumerate() {
        let bytes = n.to_be_bytes();
        let chunk_start = i * 8;
        let chunk_end = chunk_start + 8;
        result[chunk_start..chunk_end].copy_from_slice(&bytes);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_array_to_u8_array_safe() {
        let input = [0x1234, 0x5678, 0x9abc, 0xdef0];
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x56, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0xbc, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xde, 0xf0,
        ];
        let result = u64_array_to_u8_array(&input);
        assert_eq!(result, expected);
    }
}
