use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc_core::host::types::identifiers::ClientType;
use ibc_core::host::types::path::{Path, PathBytes};
use ibc_core::primitives::proto::Any;

use super::ClientState;

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(&self, consensus_state: Any) -> Result<(), ClientError> {
        todo!()
    }

    fn client_type(&self) -> ClientType {
        todo!()
    }

    fn latest_height(&self) -> ibc_core::client::types::Height {
        todo!()
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_upgrade_client(
        &self,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
        proof_upgrade_client: CommitmentProofBytes,
        proof_upgrade_consensus_state: CommitmentProofBytes,
        root: &CommitmentRoot,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn serialize_path(&self, path: Path) -> Result<PathBytes, ClientError> {
        todo!()
    }

    fn verify_membership_raw(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
        value: Vec<u8>,
    ) -> Result<(), ClientError> {
        todo!()
    }

    fn verify_non_membership_raw(
        &self,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        path: PathBytes,
    ) -> Result<(), ClientError> {
        todo!()
    }
}
