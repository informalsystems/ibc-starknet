use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{HasAddressType, HasMessageResponseType};

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

#[cgp_component {
  name: ContractInvokerComponent,
  provider: ContractInvoker,
  context: Chain,
}]
#[async_trait]
pub trait CanInvokeContract:
    HasAddressType + HasSelectorType + HasBlobType + HasMessageResponseType + HasAsyncErrorType
{
    async fn invoke_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::Selector,
        call_data: &Self::Blob,
    ) -> Result<Self::MessageResponse, Self::Error>;
}
