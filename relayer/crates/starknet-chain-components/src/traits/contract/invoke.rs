use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_test_components::chain::traits::types::address::HasAddressType;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

#[derive_component(ContractInvokerComponent, ContractInvoker<Chain>)]
#[async_trait]
pub trait CanInvokeContract:
    HasAddressType + HasSelectorType + HasBlobType + HasMessageResponseType + HasErrorType
{
    async fn invoke_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::Selector,
        call_data: &Self::Blob,
    ) -> Result<Self::MessageResponse, Self::Error>;
}
