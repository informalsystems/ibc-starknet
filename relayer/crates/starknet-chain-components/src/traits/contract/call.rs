use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::HasAddressType;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

#[cgp_component {
  name: ContractCallerComponent,
  provider: ContractCaller,
  context: Chain,
}]
#[async_trait]
pub trait CanCallContract: HasAddressType + HasSelectorType + HasBlobType + HasAsyncErrorType {
    async fn call_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::Selector,
        call_data: &Self::Blob,
    ) -> Result<Self::Blob, Self::Error>;
}
