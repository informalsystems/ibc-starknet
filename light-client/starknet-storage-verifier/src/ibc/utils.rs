use alloc::vec;
use alloc::vec::Vec;

use starknet_core::types::Felt;

pub fn serialize_to_felts(value: impl AsRef<[u8]>) -> Vec<Felt> {
    let bytes = value.as_ref();
    bytes
        .chunks(31)
        .map(|chunk| {
            let mut padded = [0u8; 32];
            padded[32 - chunk.len()..].copy_from_slice(chunk);
            Felt::from_bytes_be(&padded)
        })
        .collect()
}

pub fn serialize_byte_array(bytes: &[u8]) -> Vec<Felt> {
    let mut result = vec![Felt::ZERO];
    result.extend(serialize_to_felts(bytes));
    result.extend(vec![Felt::from(bytes.len() as u64)]);
    result
}
