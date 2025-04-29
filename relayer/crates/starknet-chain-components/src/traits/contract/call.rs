use hermes_core::chain_type_components::traits::{HasAddressType, HasHeightType};
use hermes_prelude::*;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

#[cgp_component {
  name: ContractCallerComponent,
  provider: ContractCaller,
  context: Chain,
}]
#[async_trait]
pub trait CanCallContract:
    HasAddressType + HasSelectorType + HasBlobType + HasAsyncErrorType + HasHeightType
{
    async fn call_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::Selector,
        call_data: &Self::Blob,
        height: Option<&Self::Height>,
    ) -> Result<Self::Blob, Self::Error>;
}
