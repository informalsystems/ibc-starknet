use alloc::vec::Vec;

use starknet_crypto::Felt;

pub fn packed_str_to_felt(s: &str) -> Felt {
    // Pack ASCII chars into a u256 (31 bytes max for felt252)
    let bytes = s.as_bytes();
    assert!(bytes.len() <= 31);

    let mut result = [0u8; 32];
    result[32 - bytes.len()..].copy_from_slice(bytes);

    Felt::from_bytes_be(&result)
}

pub fn parse_client_id(s: &str) -> (Felt, Felt) {
    // e.g., "07-tendermint-0"
    let parts: Vec<&str> = s.split('-').collect();
    assert!(parts.len() >= 3);

    let client_type_str = parts[1];
    let sequence: u64 = parts[2].parse().unwrap();

    let client_type = match client_type_str {
        "tendermint" => Felt::from_dec_str("3820028427552332600290323295860").unwrap(),
        "wasm" => Felt::from_dec_str("13572566809670509").unwrap(),
        _ => panic!("Unknown client type"),
    };

    (client_type, Felt::from(sequence))
}

pub fn packed_bytes_to_felt(bytes: &[u8]) -> Felt {
    assert!(bytes.len() <= 31, "prefix too long for felt");

    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    Felt::from_bytes_be(&padded)
}
