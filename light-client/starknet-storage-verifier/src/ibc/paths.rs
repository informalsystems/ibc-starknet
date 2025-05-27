use std::str::FromStr;

use ibc_core::host::types::path::Path;
use starknet_core::utils::starknet_keccak;
use starknet_crypto::{pedersen_hash, Felt};

use super::keys::{connection_key, next_sequence_key, packet_key};

pub fn to_starknet_storage_key(path: &str) -> Felt {
    let path = Path::from_str(path).unwrap();
    match path {
        Path::Connection(connection_path) => {
            let key = connection_key(connection_path.0);

            let variable_name = starknet_keccak(b"connection_ends_commitments");

            pedersen_hash(&variable_name, &key)
        }
        Path::ChannelEnd(channel_end_path) => {
            let key = next_sequence_key("channelEnds", channel_end_path.0, channel_end_path.1);

            let variable_name = starknet_keccak(b"channel_ends_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::Commitment(commitment_path) => {
            let key = packet_key(
                "commitments",
                commitment_path.port_id,
                commitment_path.channel_id,
                commitment_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_commitments_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::Ack(ack_path) => {
            let key = packet_key(
                "acks",
                ack_path.port_id,
                ack_path.channel_id,
                ack_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_acks_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::Receipt(receipt_path) => {
            let key = packet_key(
                "receipts",
                receipt_path.port_id,
                receipt_path.channel_id,
                receipt_path.sequence,
            );

            let variable_name = starknet_keccak(b"packet_receipts_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::SeqSend(seq_send_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceSend", seq_send_path.0, seq_send_path.1);

            let variable_name = starknet_keccak(b"send_sequences_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::SeqRecv(seq_recv_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceRecv", seq_recv_path.0, seq_recv_path.1);

            let variable_name = starknet_keccak(b"recv_sequences_commitments");

            pedersen_hash(&variable_name, &key)
        }

        Path::SeqAck(seq_ack_path) => {
            // Compute the Map's key
            let key = next_sequence_key("nextSequenceAck", seq_ack_path.0, seq_ack_path.1);

            let variable_name = starknet_keccak(b"ack_sequences_commitments");

            pedersen_hash(&variable_name, &key)
        }

        _ => unimplemented!(),
    }
}
