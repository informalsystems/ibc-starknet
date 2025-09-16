use hermes_core::chain_components::traits::HasHeightType;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelTrace;
use hermes_prelude::*;
use serde::de::DeserializeOwned;
use starknet::core::types::{ConfirmedBlockId, ContractStorageKeys, Felt, StorageProof};
use starknet::providers::{Provider, ProviderError};

use crate::impls::{CanValidateStorageProof, StarknetAddress};
use crate::traits::{
    HasStarknetClient, HasStorageKeyType, HasStorageProofType, StorageProofQuerier,
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
        + HasStarknetClient<Client: Provider>
        + CanRaiseAsyncError<ProviderError>,
    Chain::StorageProof: DeserializeOwned,
{
    async fn query_storage_proof(
        chain: &Chain,
        height: &u64,
        contract_address: &StarknetAddress,
        storage_keys: &[Felt],
    ) -> Result<Chain::StorageProof, Chain::Error> {
        let provider = chain.provider();

        let storage_proof = provider
            .get_storage_proof(
                ConfirmedBlockId::Number(*height),
                [],
                [contract_address.0],
                [ContractStorageKeys {
                    contract_address: contract_address.0,
                    storage_keys: storage_keys.to_vec(),
                }],
            )
            .await
            .map_err(Chain::raise_error)?;

        // Chain::verify_storage_proof(&storage_proof)?;

        Ok(storage_proof)
    }
}
