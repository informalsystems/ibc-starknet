use hermes_core::chain_components::traits::HasMessageType;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_prelude::*;

use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

#[cgp_component {
  name: InvokeContractMessageBuilderComponent,
  provider: InvokeContractMessageBuilder,
  context: Chain,
}]
pub trait CanBuildInvokeContractMessage:
    HasAddressType + HasSelectorType + HasBlobType + HasMessageType
{
    fn build_invoke_contract_message(
        &self,
        contract_address: &Self::Address,
        selector: &Self::Selector,
        call_data: &Self::Blob,
    ) -> Self::Message;
}
