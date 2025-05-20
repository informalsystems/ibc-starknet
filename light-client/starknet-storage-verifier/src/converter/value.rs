use std::str::FromStr;

use ibc_core::host::types::identifiers::{ChannelId, PortId};
use ibc_core::host::types::path::Path;
use poseidon::Poseidon3Hasher;
use starknet_core::utils::starknet_keccak;
use starknet_crypto::{pedersen_hash, Felt};

pub fn convert_storage_value(path: &str) -> Felt {
    let path = Path::from_str(path).unwrap();
    match path {
        Path::SeqAck(seq_ack_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceAck", seq_ack_path.0, seq_ack_path.1);

            let variable_name = starknet_keccak(b"ack_sequences");

            pedersen_hash(&variable_name, &key)
        }

        Path::SeqSend(seq_send_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceSend", seq_send_path.0, seq_send_path.1);

            let variable_name = starknet_keccak(b"send_sequences");

            pedersen_hash(&variable_name, &key)
        }

        Path::SeqRecv(seq_recv_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceRecv", seq_recv_path.0, seq_recv_path.1);

            let variable_name = starknet_keccak(b"recv_sequences");

            pedersen_hash(&variable_name, &key)
        }
        _ => unimplemented!(),
    }
}

pub fn next_sequence_key(prefix: &str, port_id: PortId, channel_id: ChannelId) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(prefix.as_bytes()));
    raw_path.extend(serialize_byte_array("ports".as_bytes()));
    raw_path.extend(serialize_byte_array(port_id.as_bytes()));
    raw_path.extend(serialize_byte_array("channels".as_bytes()));
    raw_path.extend(serialize_byte_array(channel_id.as_bytes()));

    Poseidon3Hasher::digest(&raw_path)
}

fn serialize_to_felts(value: impl AsRef<[u8]>) -> Vec<Felt> {
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

fn serialize_byte_array(bytes: &[u8]) -> Vec<Felt> {
    let mut result = vec![Felt::ZERO];
    result.extend(serialize_to_felts(bytes));
    result.extend(vec![Felt::from(bytes.len() as u64)]);
    result
}
