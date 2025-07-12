use alloc::string::ToString;

use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ClientType;
use ibc_core::host::types::path::{Path, PathBytes};
use ibc_core::primitives::proto::Any;
use ibc_core::primitives::Timestamp;

use crate::client_state::ClientState;

impl ClientStateCommon for ClientState {
    fn verify_consensus_state(
        &self,
        consensus_state: Any,
        host_timestamp: &Timestamp,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn client_type(&self) -> ClientType {
        "blind-001".parse().expect("Invalid client type")
    }

    fn latest_height(&self) -> Height {
        self.0.latest_height
    }

    fn validate_proof_height(&self, proof_height: Height) -> Result<(), ClientError> {
        Ok(())
    }

    fn serialize_path(&self, path: Path) -> Result<PathBytes, ClientError> {
        Ok(path.to_string().as_bytes().to_vec().into())
    }
}
