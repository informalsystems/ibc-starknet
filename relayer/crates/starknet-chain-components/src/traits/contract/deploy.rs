use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_prelude::*;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::contract_class::HasContractClassHashType;

#[cgp_component {
  name: ContractDeployerComponent,
  provider: ContractDeployer,
  context: Chain,
}]
#[async_trait]
pub trait CanDeployContract:
    HasContractClassHashType + HasBlobType + HasAddressType + HasAsyncErrorType
{
    async fn deploy_contract(
        &self,
        class_hash: &Self::ContractClassHash,
        unique: bool,
        constructor_call_data: &Self::Blob,
    ) -> Result<Self::Address, Self::Error>;
}
