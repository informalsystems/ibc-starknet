use std::str::FromStr;

use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::commitment_types::error::CommitmentError;
use ibc_core::host::types::error::DecodingError;
use ibc_core::host::types::identifiers::ClientType;
use ibc_core::host::types::path::{Path, PathBytes};
use ibc_core::primitives::proto::Any;
use ibc_core::primitives::Timestamp;
use starknet_core::types::StorageProof;
use starknet_crypto::Felt;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_storage_verifier::verifier::verify_starknet_merkle_proof;

use super::ClientState;

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(
        &self,
        consensus_state: Any,
        host_timestamp: &Timestamp,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn client_type(&self) -> ClientType {
        "blind-001".parse().unwrap()
    }

    fn latest_height(&self) -> Height {
        self.0.latest_height
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        Ok(())
    }

    fn verify_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn serialize_path(&self, path: Path) -> Result<PathBytes, ClientError> {
        Ok(path.to_string().as_bytes().to_vec().into())
    }

    fn verify_membership_raw(
        &self,
        _prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        let path_bytes = path.into_vec();
        let processed_path = str::from_utf8(path_bytes.as_slice())
            .map_err(|e| ClientError::Decoding(DecodingError::StrUtf8(e)))?;
        let felt_path = ibc_path_to_storage_key(Path::from_str(processed_path).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidRawData {
                description: e.to_string(),
            })
        })?);

        let storage_proof: StorageProof = serde_json::from_slice(proof.as_ref()).map_err(|e| {
            ClientError::Decoding(DecodingError::InvalidJson {
                description: e.to_string(),
            })
        })?;

        let root = Felt::from_bytes_be_slice(root.as_bytes());

        let value = Felt::from_bytes_be_slice(value.as_slice());

        verify_starknet_merkle_proof(&storage_proof.classes_proof, root, felt_path, value).map_err(
            |e| ClientError::FailedICS23Verification(CommitmentError::FailedToVerifyMembership),
        )
    }

    fn verify_non_membership_raw(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
    ) -> Result<(), ClientError> {
        Ok(())
    }
}
