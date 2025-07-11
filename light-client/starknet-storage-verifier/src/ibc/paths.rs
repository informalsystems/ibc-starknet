use ibc_core::host::types::path::Path;
use starknet_core::types::Felt;
use starknet_crypto_lib::StarknetCryptoFunctions;

use crate::ibc::keys::{connection_key, next_sequence_key, packet_key};
use crate::storage::key::{starknet_storage_key, KeyPart};

pub fn ibc_path_to_storage_key<C: StarknetCryptoFunctions>(crypto_lib: &C, path: Path) -> Felt {
    match path {
        Path::Connection(connection_path) => {
            let key = connection_key(crypto_lib, connection_path.0);

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"connection_ends_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::ChannelEnd(channel_end_path) => {
            let key = next_sequence_key(
                crypto_lib,
                "channelEnds",
                channel_end_path.0,
                channel_end_path.1,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"channel_ends_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::Commitment(commitment_path) => {
            let key = packet_key(
                crypto_lib,
                "commitments",
                commitment_path.port_id,
                commitment_path.channel_id,
                commitment_path.sequence,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"packet_commitments_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::Ack(ack_path) => {
            let key = packet_key(
                crypto_lib,
                "acks",
                ack_path.port_id,
                ack_path.channel_id,
                ack_path.sequence,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"packet_acks_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::Receipt(receipt_path) => {
            let key = packet_key(
                crypto_lib,
                "receipts",
                receipt_path.port_id,
                receipt_path.channel_id,
                receipt_path.sequence,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"packet_receipts_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::SeqSend(seq_send_path) => {
            // Compute the Map's key
            let key = next_sequence_key(
                crypto_lib,
                "nextSequenceSend",
                seq_send_path.0,
                seq_send_path.1,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"send_sequences_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::SeqRecv(seq_recv_path) => {
            // Compute the Map's key
            let key = next_sequence_key(
                crypto_lib,
                "nextSequenceRecv",
                seq_recv_path.0,
                seq_recv_path.1,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"recv_sequences_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        Path::SeqAck(seq_ack_path) => {
            // Compute the Map's key
            let key = next_sequence_key(
                crypto_lib,
                "nextSequenceAck",
                seq_ack_path.0,
                seq_ack_path.1,
            );

            starknet_storage_key(
                crypto_lib,
                [
                    KeyPart::Field(b"ack_sequences_commitments"),
                    KeyPart::Map(key),
                ],
            )
        }

        // Note: `proof_client` and `proof_consensus` are deprecated by ibc-go.
        // So, we do not support them.
        _ => unimplemented!(),
    }
}
