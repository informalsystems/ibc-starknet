use hermes_core::chain_components::traits::HasHeightType;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_prelude::*;

use crate::traits::{HasStorageKeyType, HasStorageProofType};

#[cgp_component {
    provider: StorageProofQuerier,
}]
#[async_trait]
pub trait CanQueryStorageProof:
    HasHeightType + HasAddressType + HasStorageKeyType + HasStorageProofType + HasAsyncErrorType
{
    async fn query_storage_proof(
        &self,
        height: &Self::Height,
        contract_address: &Self::Address,
        storage_keys: &[Self::StorageKey],
    ) -> Result<Self::StorageProof, Self::Error>;
}
