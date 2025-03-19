/// Converts a vector of u8 values to a slice of u32 values in big-endian order.
/// This is used for proof membership verification in Cairo because the sha256
/// hasher used in Cairo outputs [u32; 8].
pub fn from_vec_u8_to_be_u32_slice(u8_values: Vec<u8>) -> Result<[u32; 8], String> {
    let u8_slice: [u8; 32] = u8_values
        .try_into()
        .map_err(|e| format!("failed to convert values `{e:?}` to [u8; 32]"))?;

    let mut u32_slice: [u32; 8] = [0; 8];

    for (i, u32_value) in u32_slice.iter_mut().enumerate() {
        let offset = i * 4;
        *u32_value = u32::from_be_bytes([
            u8_slice[offset],
            u8_slice[offset + 1],
            u8_slice[offset + 2],
            u8_slice[offset + 3],
        ]);
    }

    Ok(u32_slice)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Tests that the retrieved Vec<u8> value from Cosmos is correctly converted to
    /// the expected [u32; 8] value for Cairo.
    #[test]
    fn test_convert_cairo_sha() {
        // `data` and `expected` are picked from the Scarb documentation to match its implementation:
        // https://docs.swmansion.com/scarb/corelib/core-sha256.html
        let data = "Hello";
        let expected: [u32; 8] = [
            0x185f8db3, 0x2271fe25, 0xf561a6fc, 0x938b2e26, 0x4306ec30, 0x4eda5180, 0x7d17648,
            0x26381969,
        ];

        let hash = sha256::digest(data.as_bytes());
        let hash_u8_array: Vec<u8> = hex::decode(hash).expect("Invalid hex string");

        let hash_u32_slice = from_vec_u8_to_be_u32_slice(hash_u8_array).unwrap();

        assert_eq!(hash_u32_slice, expected);
    }
}
