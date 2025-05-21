use std::str::FromStr;

use ibc_core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc_core::host::types::path::Path;
use poseidon::Poseidon3Hasher;
use starknet_core::utils::starknet_keccak;
use starknet_crypto::{pedersen_hash, Felt};

pub fn convert_storage_value(path: &str) -> Felt {
    let path = Path::from_str(path).unwrap();
    match path {
        Path::ChannelEnd(channel_end_path) => {
            let key = next_sequence_key("channelEnds", channel_end_path.0, channel_end_path.1);

            let variable_name = starknet_keccak(b"channel_ends");

            pedersen_hash(&variable_name, &key)
        }

        Path::Commitment(commitment_path) => {
            let key = packet_key(
                "commitments",
                commitment_path.port_id,
                commitment_path.channel_id,
                commitment_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::Ack(ack_path) => {
            let key = packet_key(
                "acks",
                ack_path.port_id,
                ack_path.channel_id,
                ack_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_acks");

            pedersen_hash(&variable_name, &key)
        }

        Path::Receipt(receipt_path) => {
            let key = packet_key(
                "receipts",
                receipt_path.port_id,
                receipt_path.channel_id,
                receipt_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_receipts");

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

        Path::SeqAck(seq_ack_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceAck", seq_ack_path.0, seq_ack_path.1);

            let variable_name = starknet_keccak(b"ack_sequences");

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

pub fn packet_key(
    prefix: &str,
    port_id: PortId,
    channel_id: ChannelId,
    sequence: Sequence,
) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(prefix.as_bytes()));
    raw_path.extend(serialize_byte_array("ports".as_bytes()));
    raw_path.extend(serialize_byte_array(port_id.as_bytes()));
    raw_path.extend(serialize_byte_array("channels".as_bytes()));
    raw_path.extend(serialize_byte_array(channel_id.as_bytes()));
    raw_path.extend(serialize_byte_array("sequences".as_bytes()));
    raw_path.extend(serialize_to_felts(sequence.to_vec().as_slice()));

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
