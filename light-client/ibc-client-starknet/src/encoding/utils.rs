use starknet_core::types::Felt;
use starknet_core::utils::cairo_short_string_to_felt;

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
    let (client_type_str, sequence_str) = s.rsplit_once('-').expect("Invalid client ID format");

    let client_type = match client_type_str {
        "07-tendermint" | "08-wasm" => cairo_short_string_to_felt(client_type_str).unwrap(), // short_string!(x)
        _ => panic!("Unknown client type"),
    };

    let sequence: u64 = sequence_str.parse().unwrap();

    (client_type, Felt::from(sequence))
}

pub fn packed_bytes_to_felt(bytes: &[u8]) -> Felt {
    assert!(bytes.len() <= 31, "prefix too long for felt");

    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    Felt::from_bytes_be(&padded)
}
