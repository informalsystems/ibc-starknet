use hermes_core::chain_components::traits::HasMessageType;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::impls::{StarknetAddress, StarknetMessage};
use crate::traits::{
    HasBlobType, HasSelectorType, InvokeContractMessageBuilder,
    InvokeContractMessageBuilderComponent,
};

pub struct BuildInvokeContractCall;

#[cgp_provider(InvokeContractMessageBuilderComponent)]
impl<Chain> InvokeContractMessageBuilder<Chain> for BuildInvokeContractCall
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMessageType<Message = StarknetMessage>,
{
    fn build_invoke_contract_message(
        _chain: &Chain,
        contract_address: &StarknetAddress,
        entry_point_selector: &Felt,
        calldata: &Vec<Felt>,
    ) -> StarknetMessage {
        StarknetMessage::new(contract_address.0, *entry_point_selector, calldata.clone())
    }
}
