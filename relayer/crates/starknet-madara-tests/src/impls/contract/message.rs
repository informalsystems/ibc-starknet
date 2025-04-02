use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::contract::message::{
    InvokeContractMessageBuilder, InvokeContractMessageBuilderComponent,
};
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet_v13::core::types::{Call, Felt};

use crate::types::StarknetMessage;

#[cgp_new_provider(InvokeContractMessageBuilderComponent)]
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
        let call = Call {
            to: **contract_address,
            selector: *entry_point_selector,
            calldata: calldata.clone(),
        };

        StarknetMessage::new(call)
    }
}
