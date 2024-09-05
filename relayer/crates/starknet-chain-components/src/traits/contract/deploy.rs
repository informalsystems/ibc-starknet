use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::HasAddressType;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::contract_class::HasContractClassHashType;

#[derive_component(ContractDeployerComponent, ContractDeployer<Chain>)]
#[async_trait]
pub trait CanDeployContract:
    HasContractClassHashType + HasBlobType + HasAddressType + HasErrorType
{
    async fn deploy_contract(
        &self,
        class_hash: &Self::ContractClassHash,
        unique: bool,
        constructor_call_data: &Self::Blob,
    ) -> Result<Self::Address, Self::Error>;
}
