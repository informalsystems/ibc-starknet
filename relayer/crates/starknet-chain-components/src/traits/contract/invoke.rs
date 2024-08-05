use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_test_components::chain::traits::types::address::HasAddressType;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

#[derive_component(ContractInvokerComponent, ContractInvoker<Chain>)]
#[async_trait]
pub trait CanInvokeContract:
    HasAddressType + HasMethodSelectorType + HasBlobType + HasEventType + HasErrorType
{
    async fn invoke_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::MethodSelector,
        call_data: &Self::Blob,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}
