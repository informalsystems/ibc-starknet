use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::str::FromStr;

use ibc_client_cw::context::CwClientValidation;
use ibc_core::channel::types::proto::v1::Channel;
use ibc_core::client::context::client_state::ClientStateValidation;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Status;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::commitment_types::error::CommitmentError;
use ibc_core::connection::types::proto::v1::ConnectionEnd;
use ibc_core::host::types::error::DecodingError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{Path, PathBytes};
use ibc_core::primitives::proto::Any;
use prost::Message;
use starknet_block_verifier::{StarknetCryptoEmpty as StarknetCrypto, StarknetCryptoFunctions};
use starknet_core::types::{Felt, StorageProof};
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_storage_verifier::validate::validate_storage_proof;
use starknet_storage_verifier::verifier::verify_starknet_storage_proof;

use super::ClientState;
use crate::encoding::channel::channel_to_felts;
use crate::encoding::connection::connection_end_to_felts;
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
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
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
        let felt_value = get_felt_from_value::<StarknetCrypto>(&value, &processed_path)?;
        let felt_path = ibc_path_to_storage_key::<StarknetCrypto>(processed_path);

        let storage_proof: StorageProof = serde_json::from_slice(proof.as_ref()).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidJson {
                description: e.to_string(),
            })
        })?;

        validate_storage_proof::<StarknetCrypto>(&storage_proof).map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // TODO: Verify that the root matches the one in the storage proof

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
        let felt_path = ibc_path_to_storage_key::<StarknetCrypto>(processed_path);

        let storage_proof: StorageProof = serde_json::from_slice(proof.as_ref()).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidJson {
                description: e.to_string(),
            })
        })?;

        validate_storage_proof::<StarknetCrypto>(&storage_proof).map_err(|e| {
            ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership)
        })?;

        // TODO: Verify that the root matches the one in the storage proof

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
    value: &Vec<u8>,
    path: &Path,
) -> Result<Felt, ClientError> {
    match path {
        Path::Connection(_) => {
            let connection_end = ConnectionEnd::decode(value.as_slice()).unwrap();
            let felts = connection_end_to_felts(&connection_end);

            Ok(C::poseidon_hash_many(&felts))
        }
        Path::ChannelEnd(_) => {
            let channel = Channel::decode(value.as_slice()).unwrap();
            let felts = channel_to_felts(&channel);

            Ok(C::poseidon_hash_many(&felts))
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

            Ok(C::poseidon_hash_many(&felts))
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

            Ok(C::poseidon_hash_many(&felts))
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

            Ok(C::poseidon_hash_many(&felts))
        }
        _ => {
            let mut text = String::new();
            write!(&mut text, "Unknown path type: {path}").expect("Failed to write to string");
            Err(ClientError::ClientSpecific { description: text })
        }
    }
}
