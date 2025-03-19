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
