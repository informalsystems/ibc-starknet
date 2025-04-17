use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use serde::de::DeserializeOwned;
use serde::Serialize;
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;
use crate::traits::json_rpc::CanSendJsonRpcRequest;
use crate::traits::queries::storage_proof::{StorageProofQuerier, StorageProofQuerierComponent};
use crate::traits::types::storage_proof::{HasStorageKeyType, HasStorageProofType};

#[cgp_new_provider(StorageProofQuerierComponent)]
impl<Chain> StorageProofQuerier<Chain> for QueryStarknetStorageProof
where
    Chain: HasHeightType<Height = u64>
        + HasAddressType<Address = StarknetAddress>
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType
        + CanSendJsonRpcRequest<QueryStorageProofRequest, Chain::StorageProof>,
    Chain::StorageProof: DeserializeOwned,
{
    async fn query_storage_proof(
        chain: &Chain,
        height: &u64,
        contract_address: &StarknetAddress,
        storage_keys: &[Felt],
    ) -> Result<Chain::StorageProof, Chain::Error> {
        let request = QueryStorageProofRequest {
            block_id: "latest",
            contract_storage_keys: vec![ContractStorageKey {
                contract_address: contract_address.0,
                storage_keys: Vec::from(storage_keys),
            }],
        };

        let storage_proof = chain
            .send_json_rpc_request("starknet_getStorageProof", &request)
            .await?;

        Ok(storage_proof)
    }
}

#[derive(Serialize)]
pub struct QueryStorageProofRequest {
    pub block_id: &'static str,
    pub contract_storage_keys: Vec<ContractStorageKey>,
}

#[derive(Serialize)]
pub struct ContractStorageKey {
    pub contract_address: Felt,
    pub storage_keys: Vec<Felt>,
}
