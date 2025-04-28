use cgp::prelude::*;
use hermes_chain_type_components::traits::HasAddressType;
use hermes_core::chain_components::traits::HasMessageType;
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::contract::message::{
    InvokeContractMessageBuilder, InvokeContractMessageBuilderComponent,
};
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

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
