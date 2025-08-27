use hermes_core::chain_components::traits::HasHeightType;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelTrace;
use hermes_prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use starknet::core::types::{BlockId, Felt, StorageProof};

use crate::impls::{CanValidateStorageProof, StarknetAddress};
use crate::traits::{
    CanSendJsonRpcRequest, HasStorageKeyType, HasStorageProofType, StorageProofQuerier,
    StorageProofQuerierComponent,
};

#[cgp_new_provider(StorageProofQuerierComponent)]
impl<Chain> StorageProofQuerier<Chain> for QueryStarknetStorageProof
where
    Chain: HasHeightType<Height = u64>
        + HasAddressType<Address = StarknetAddress>
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanValidateStorageProof
        + CanLog<LevelTrace>
        + CanSendJsonRpcRequest<QueryStorageProofRequest, Chain::StorageProof>
        + CanRaiseError<serde_json::Error>,
    Chain::StorageProof: DeserializeOwned,
{
    async fn query_storage_proof(
        chain: &Chain,
        height: &u64,
        contract_address: &StarknetAddress,
        storage_keys: &[Felt],
    ) -> Result<Chain::StorageProof, Chain::Error> {
        let request = QueryStorageProofRequest {
            block_id: BlockId::Number(*height),
            contract_addresses: vec![contract_address.0],
            contracts_storage_keys: vec![ContractStorageKey {
                contract_address: contract_address.0,
                storage_keys: Vec::from(storage_keys),
            }],
        };

        let storage_proof = chain
            .send_json_rpc_request("starknet_getStorageProof", &request)
            .await?;

        let storage_proof_str =
            serde_json::to_string_pretty(&storage_proof).map_err(Chain::raise_error)?;

        chain
            .log(
                &format!("fetched storage proof: {storage_proof_str}"),
                &LevelTrace,
            )
            .await;

        // Chain::verify_storage_proof(&storage_proof)?;

        Ok(storage_proof)
    }
}

#[derive(Serialize)]
pub struct QueryStorageProofRequest {
    pub block_id: BlockId,
    pub contract_addresses: Vec<Felt>,
    pub contracts_storage_keys: Vec<ContractStorageKey>,
}

#[derive(Serialize)]
pub struct ContractStorageKey {
    pub contract_address: Felt,
    pub storage_keys: Vec<Felt>,
}
