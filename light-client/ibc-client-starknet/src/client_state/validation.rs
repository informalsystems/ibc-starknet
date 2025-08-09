use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;
use core::str::FromStr;

use cgp::core::component::UseContext;
use hermes_cosmos_encoding_components::impls::ConvertIbcAny;
use hermes_encoding_components::impls::ConvertVia;
use hermes_encoding_components::traits::Converter;
use ibc_client_cw::context::CwClientValidation;
use ibc_client_starknet_types::header::StarknetHeader;
use ibc_client_starknet_types::{StarknetClientState, StarknetConsensusState};
use ibc_core::channel::types::proto::v1::Channel;
use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::context::prelude::ClientStateCommon;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Status;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::commitment_types::error::CommitmentError;
use ibc_core::connection::types::proto::v1::ConnectionEnd;
use ibc_core::host::types::error::DecodingError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{
    Path, PathBytes, UpgradeClientStatePath, UpgradeConsensusStatePath,
};
use ibc_core::primitives::proto::{Any, Protobuf};
use prost::Message;
use prost_types::Any as ProstAny;
use starknet_core::types::{Felt, StorageProof};
use starknet_crypto_lib::{StarknetCryptoFunctions, StarknetCryptoLib};
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_storage_verifier::validate::validate_storage_proof;
use starknet_storage_verifier::verifier::{
    verify_starknet_contract_proof, verify_starknet_global_contract_root,
    verify_starknet_storage_proof,
};

use super::ClientState;
use crate::encoding::channel::channel_to_felts;
use crate::encoding::connection::connection_end_to_felts;
use crate::encoding::context::StarknetLightClientEncoding;
use crate::ConsensusState;

impl<'a, V> ClientStateValidation<V> for ClientState
where
    V: CwClientValidation<'a, ClientStateRef = Self, ConsensusStateRef = ConsensusState>,
{
    fn verify_client_message(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        let starknet_crypto_cw = StarknetCryptoLib;

        let header: StarknetHeader = <ConvertVia<ProstAny, ConvertIbcAny, UseContext>>::convert(
            &StarknetLightClientEncoding,
            &client_message,
        )?;

        let StarknetHeader {
            block_header,
            final_height,
            block_signature,
            storage_proof,
        } = header;

        // TODO(rano): use a proper error type
        // this is to make sure after a schedule upgrade, the client can't be updated after final_height.
        // this way, any packet after the final_height will be rejected.
        // only way to resume the client is to remove the scheduled upgrade on starknet after the upgrade is finished.
        assert!(final_height == 0 || block_header.block_number <= final_height);

        let sequencer_public_key = Felt::from_bytes_be_slice(&self.0.sequencer_public_key);
        let ibc_contract_address = Felt::from_bytes_be_slice(&self.0.ibc_contract_address);

        // 1. verify the block header
        block_header
            .verify_signature(&starknet_crypto_cw, &block_signature, &sequencer_public_key)
            .map_err(|e| ClientError::FailedToVerifyHeader {
                description: e.to_string(),
            })?;

        // 2. validate the storage proof with correct merkle nodes
        validate_storage_proof(&starknet_crypto_cw, &storage_proof).map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // 3. verify the global contract storage root is correct
        let global_contract_trie_root = verify_starknet_global_contract_root(
            &starknet_crypto_cw,
            &storage_proof,
            block_header.state_root,
        )
        .map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // 4. verify the contract storage root is correct
        let contract_root = verify_starknet_contract_proof(
            &starknet_crypto_cw,
            &storage_proof,
            global_contract_trie_root,
            ibc_contract_address,
        )
        .map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        verify_starknet_storage_proof(
            &storage_proof,
            contract_root,
            // expansion of: selector!("final_height")
            // to avoid import of: starknet_macros
            Felt::from_raw([
                282283167788747436,
                16778837309615584552,
                17246355766618278593,
                8468359089124617139,
            ]),
            final_height.into(),
        )
        .map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        Ok(())
    }

    fn check_for_misbehaviour(
        &self,
        ctx: &V,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<bool, ClientError> {
        Ok(false)
    }

    fn status(&self, ctx: &V, client_id: &ClientId) -> Result<Status, ClientError> {
        Ok(Status::Active)
    }

    fn check_substitute(&self, ctx: &V, substitute_client_state: Any) -> Result<(), ClientError> {
        Ok(())
    }

    fn verify_upgrade_client(
        &self,
        ctx: &V,
        upgraded_client_state_any: Any,
        upgraded_consensus_state_any: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        let starknet_crypto_cw = StarknetCryptoLib;

        let upgraded_client_state = V::ClientStateRef::try_from(upgraded_client_state_any.clone())?;

        let upgraded_consensus_state =
            V::ConsensusStateRef::try_from(upgraded_consensus_state_any.clone())?;

        let latest_height = self.latest_height();
        let final_height = self.0.final_height;

        // TODO(rano): use proper error type
        // the client must be updated till the final height.
        assert!(latest_height.revision_height() == final_height);

        let upgraded_client_path =
            UpgradeClientStatePath::new_with_default_path(final_height).into();

        let upgraded_consensus_path =
            UpgradeConsensusStatePath::new_with_default_path(final_height).into();

        // commitment root is: contract_storage_root.to_bytes_be()
        let contract_root = Felt::from_bytes_be_slice(root.as_bytes());

        {
            let felt_value = get_felt_from_value(
                &starknet_crypto_cw,
                &upgraded_client_state_any.value,
                &upgraded_client_path,
            )?;
            let felt_path = ibc_path_to_storage_key(&starknet_crypto_cw, upgraded_client_path);

            let storage_proof: StorageProof = serde_json::from_slice(proof_upgrade_client.as_ref())
                .map_err(|e| {
                    ClientError::Decoding(DecodingError::InvalidJson {
                        description: e.to_string(),
                    })
                })?;

            validate_storage_proof(&starknet_crypto_cw, &storage_proof).map_err(|e| {
                ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
            })?;

            verify_starknet_storage_proof(&storage_proof, contract_root, felt_path, felt_value)
                .map_err(|e| {
                    ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
                })?;
        }

        {
            {
                let felt_value = get_felt_from_value(
                    &starknet_crypto_cw,
                    &upgraded_consensus_state_any.value,
                    &upgraded_consensus_path,
                )?;
                let felt_path =
                    ibc_path_to_storage_key(&starknet_crypto_cw, upgraded_consensus_path);

                let storage_proof: StorageProof = serde_json::from_slice(
                    proof_upgrade_consensus_state.as_ref(),
                )
                .map_err(|e| {
                    ClientError::Decoding(DecodingError::InvalidJson {
                        description: e.to_string(),
                    })
                })?;

                validate_storage_proof(&starknet_crypto_cw, &storage_proof).map_err(|e| {
                    ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
                })?;

                verify_starknet_storage_proof(&storage_proof, contract_root, felt_path, felt_value)
                    .map_err(|e| {
                        ClientError::FailedICS23Verification(
                            CommitmentError::FailedToVerifyMembership,
                        )
                    })?;
            }
        }

        Ok(())
    }

    fn verify_membership_raw(
        &self,
        ctx: &V,
        _prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        let starknet_crypto_cw = StarknetCryptoLib;

        let path_bytes = path.into_vec();
        let processed_path = Path::from_str(
            alloc::str::from_utf8(path_bytes.as_ref())
                .map_err(|e| ClientError::Decoding(DecodingError::StrUtf8(e)))?,
        )
        .map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidRawData {
                description: e.to_string(),
            })
        })?;
        let felt_value = get_felt_from_value(&starknet_crypto_cw, &value, &processed_path)?;
        let felt_path = ibc_path_to_storage_key(&starknet_crypto_cw, processed_path);

        let storage_proof: StorageProof = serde_json::from_slice(proof.as_ref()).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidJson {
                description: e.to_string(),
            })
        })?;

        validate_storage_proof(&starknet_crypto_cw, &storage_proof).map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // commitment root is: contract_storage_root.to_bytes_be()
        let contract_root = Felt::from_bytes_be_slice(root.as_bytes());

        verify_starknet_storage_proof(&storage_proof, contract_root, felt_path, felt_value)
            .map_err(|e| {
                ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
            })?;

        Ok(())
    }

    fn verify_non_membership_raw(
        &self,
        ctx: &V,
        _prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
    ) -> Result<(), ClientError> {
        let starknet_crypto_cw = StarknetCryptoLib;

        let path_bytes = path.into_vec();
        let processed_path = Path::from_str(
            alloc::str::from_utf8(path_bytes.as_ref())
                .map_err(|e| ClientError::Decoding(DecodingError::StrUtf8(e)))?,
        )
        .map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidRawData {
                description: e.to_string(),
            })
        })?;
        let felt_path = ibc_path_to_storage_key(&starknet_crypto_cw, processed_path);

        let storage_proof: StorageProof = serde_json::from_slice(proof.as_ref()).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidJson {
                description: e.to_string(),
            })
        })?;

        validate_storage_proof(&starknet_crypto_cw, &storage_proof).map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // commitment root is: contract_storage_root.to_bytes_be()
        let contract_root = Felt::from_bytes_be_slice(root.as_bytes());

        // For non-membership proof, the expected value is a zero value
        let felt_value = Felt::ZERO;

        verify_starknet_storage_proof(&storage_proof, contract_root, felt_path, felt_value)
            .map_err(|e| {
                ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
            })?;

        Ok(())
    }
}

fn get_felt_from_value<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    value: &Vec<u8>,
    path: &Path,
) -> Result<Felt, ClientError> {
    match path {
        Path::Connection(_) => {
            let connection_end = ConnectionEnd::decode(value.as_slice())
                .map_err(|e| ClientError::Decoding(e.into()))?;
            let felts = connection_end_to_felts(&connection_end);

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::ChannelEnd(_) => {
            let channel =
                Channel::decode(value.as_slice()).map_err(|e| ClientError::Decoding(e.into()))?;
            let felts = channel_to_felts(&channel);

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::Commitment(_) => {
            assert!(value.len() == 32, "commitment must be 32 bytes");
            let value_in_u32: Vec<u32> = value
                .chunks(4)
                .map(|chunk| {
                    let mut padded = [0u8; 4];
                    padded[..chunk.len()].copy_from_slice(chunk);
                    u32::from_be_bytes(padded)
                })
                .collect();
            let felts = value_in_u32
                .iter()
                .map(|v| Felt::from(*v))
                .collect::<Vec<Felt>>();

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::Receipt(_) => {
            assert!(value.len() == 32, "receipt must be 32 bytes");
            let value_in_u32: Vec<u32> = value
                .chunks(4)
                .map(|chunk| {
                    let mut padded = [0u8; 4];
                    padded[..chunk.len()].copy_from_slice(chunk);
                    u32::from_be_bytes(padded)
                })
                .collect();
            let felts = value_in_u32
                .iter()
                .map(|v| Felt::from(*v))
                .collect::<Vec<Felt>>();

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::Ack(_) => {
            assert!(value.len() == 32, "acknowledgement must be 32 bytes");
            let value_in_u32: Vec<u32> = value
                .chunks(4)
                .map(|chunk| {
                    let mut padded = [0u8; 4];
                    padded[..chunk.len()].copy_from_slice(chunk);
                    u32::from_be_bytes(padded)
                })
                .collect();
            let felts = value_in_u32
                .iter()
                .map(|v| Felt::from(*v))
                .collect::<Vec<Felt>>();

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::UpgradeClientState(_) => {
            let upgrade_client_state: ClientState = ClientState::decode_vec(value.as_slice())
                .map_err(|e| ClientError::Decoding(e.into()))?;

            // TODO(rano): cairo serialize the client state
            let mut felts = vec![];

            {
                let StarknetClientState {
                    latest_height,
                    final_height,
                    chain_id,
                    sequencer_public_key,
                    ibc_contract_address,
                } = upgrade_client_state.0;

                let chain_id_bytes = chain_id.as_str().as_bytes();
                felts.push(Felt::from(latest_height.revision_number()));
                felts.push(Felt::from(latest_height.revision_height()));
                felts.push(Felt::from(final_height));
                felts.push(Felt::from(chain_id_bytes.len() as u32));
                for byte in chain_id_bytes {
                    felts.push(Felt::from(*byte));
                }
                felts.push(Felt::from_bytes_be_slice(&sequencer_public_key));
                felts.push(Felt::from_bytes_be_slice(&ibc_contract_address));
            }

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        Path::UpgradeConsensusState(_) => {
            let upgrade_consensus_state: ConsensusState = ConsensusState::decode(value.as_slice())
                .map_err(|e| ClientError::Decoding(e.into()))?;

            let mut felts = vec![];

            {
                let StarknetConsensusState { root, time } = upgrade_consensus_state.0;

                felts.push(Felt::from_bytes_be_slice(root.as_bytes()));
                felts.push(Felt::from(time.nanoseconds()));
            }

            Ok(crypto_lib.poseidon_hash_many(&felts))
        }
        _ => {
            let mut text = String::new();
            write!(&mut text, "Unknown path type: {path}").expect("Failed to write to string");
            Err(ClientError::ClientSpecific { description: text })
        }
    }
}

#[cfg(test)]
mod tests {
    use ibc_core::client::types::Height;
    use ibc_core::primitives::Timestamp;

    use super::*;

    #[test]
    fn test_upgraded_client_state_key() {
        let client_state = StarknetClientState {
            latest_height: Height::new(1, 2).unwrap(),
            final_height: 3,
            chain_id: "test_chain".parse().unwrap(),
            sequencer_public_key: Felt::from(0x12345).to_bytes_be().to_vec(),
            ibc_contract_address: Felt::from_hex("0x1234567890abcdef1234567890abcdef")
                .unwrap()
                .to_bytes_be()
                .to_vec(),
        };
        let mut felts = vec![];

        {
            let StarknetClientState {
                latest_height,
                final_height,
                chain_id,
                sequencer_public_key,
                ibc_contract_address,
            } = client_state;

            let chain_id_bytes = chain_id.as_str().as_bytes();

            felts.push(Felt::from(latest_height.revision_number()));
            felts.push(Felt::from(latest_height.revision_height()));
            felts.push(Felt::from(final_height));
            felts.push(Felt::from(chain_id_bytes.len() as u32));
            for byte in chain_id_bytes {
                felts.push(Felt::from(*byte));
            }
            felts.push(Felt::from_bytes_be_slice(&sequencer_public_key));
            felts.push(Felt::from_bytes_be_slice(&ibc_contract_address));
        }

        let commitment = StarknetCryptoLib.poseidon_hash_many(&felts);

        assert_eq!(
            commitment,
            Felt::from_hex("0x362d1520e188b61477f5863e9e942f0119c01188c7f88cce0188f72a41602ea")
                .unwrap(),
        );
    }

    #[test]
    fn test_upgraded_consensus_state_key() {
        let consensus_state = StarknetConsensusState {
            root: Felt::from_hex("0x12345")
                .unwrap()
                .to_bytes_be()
                .to_vec()
                .into(),
            time: Timestamp::from_nanoseconds(67890),
        };
        let mut felts = vec![];

        {
            let StarknetConsensusState { root, time } = consensus_state;

            felts.push(Felt::from_bytes_be_slice(root.as_bytes()));
            felts.push(Felt::from(time.nanoseconds()));
        }

        let commitment = StarknetCryptoLib.poseidon_hash_many(&felts);

        assert_eq!(
            commitment,
            Felt::from_hex("0x63bbd344c107adb53145910cb75ae2901f9bdea80967dfef54bed58ce4fff15")
                .unwrap(),
        );
    }
}
