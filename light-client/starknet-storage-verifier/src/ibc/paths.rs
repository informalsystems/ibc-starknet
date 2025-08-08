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

        Path::UpgradeClientState(upgrade_client_state_path) => starknet_storage_key(
            crypto_lib,
            [
                KeyPart::Field(b"upgraded_client_state_commitments"),
                KeyPart::Map(upgrade_client_state_path.height.into()),
            ],
        ),

        Path::UpgradeConsensusState(upgrade_consensus_state_path) => starknet_storage_key(
            crypto_lib,
            [
                KeyPart::Field(b"upgraded_consensus_state_commitments"),
                KeyPart::Map(upgrade_consensus_state_path.height.into()),
            ],
        ),

        // Note: ibc-go deprecates the use of client_proof and consensus_proof.
        // We return a dummy value for these paths for API compatibility reasons.
        Path::ClientState(client_state_path) => {
            // TODO
            Felt::ZERO
        }

        Path::ClientConsensusState(client_consensus_state_path) => {
            // TODO
            Felt::ZERO
        }

        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use ibc_core::host::types::path::{UpgradeClientStatePath, UpgradeConsensusStatePath};
    use starknet_crypto_lib::StarknetCryptoLib;

    use super::*;

    #[test]
    fn test_upgraded_states_path() {
        let upgraded_client_state_path = UpgradeClientStatePath {
            upgrade_path: String::new(),
            height: 42,
        };

        let upgraded_consensus_state_path = UpgradeConsensusStatePath {
            upgrade_path: String::new(),
            height: 42,
        };

        let crypto_lib = StarknetCryptoLib;

        let client_state_key =
            ibc_path_to_storage_key(&crypto_lib, upgraded_client_state_path.into());

        let consensus_state_key =
            ibc_path_to_storage_key(&crypto_lib, upgraded_consensus_state_path.into());

        assert_eq!(
            client_state_key,
            Felt::from_hex("0x7f1877168ebc2b7ec579aa0f1514007124ad2c19fe35a56f3b12d2c68718a44")
                .unwrap()
        );

        assert_eq!(
            consensus_state_key,
            Felt::from_hex("0xef005e48e802e8403a09622b8ffd8299020c511293a5ed773b0f5d80ab81b9")
                .unwrap()
        );
    }
}
