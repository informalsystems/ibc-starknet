use cgp_core::prelude::*;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

#[derive_component(ContractCallerComponent, ContractCaller<Chain>)]
#[async_trait]
pub trait CanCallContract:
    HasAddressType + HasMethodSelectorType + HasBlobType + HasErrorType
{
    async fn call_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::MethodSelector,
        call_data: &Self::Blob,
    ) -> Result<Self::Blob, Self::Error>;
}
