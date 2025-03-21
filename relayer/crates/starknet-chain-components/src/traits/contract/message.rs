use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;

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
