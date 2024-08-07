use cgp_core::prelude::*;

use crate::traits::types::contract_class::{HasContractClassHashType, HasContractClassType};

#[derive_component(ContractDeclarerComponent, ContractDeclarer<Chain>)]
#[async_trait]
pub trait CanDeclareContract:
    HasContractClassType + HasContractClassHashType + HasErrorType
{
    async fn declare_contract(
        &self,
        contract_class: &Self::ContractClass,
    ) -> Result<Self::ContractClassHash, Self::Error>;
}
