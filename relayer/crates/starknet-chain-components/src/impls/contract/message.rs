use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet::accounts::Call;
use starknet::core::types::Felt;

use crate::impls::types::message::StarknetMessage;
use crate::traits::contract::message::InvokeContractMessageBuilder;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

pub struct BuildInvokeContractCall;

impl<Chain> InvokeContractMessageBuilder<Chain> for BuildInvokeContractCall
where
    Chain: HasAddressType<Address = Felt>
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMessageType<Message = StarknetMessage>,
{
    fn build_invoke_contract_message(
        _chain: &Chain,
        contract_address: &Felt,
        entry_point_selector: &Felt,
        calldata: &Vec<Felt>,
    ) -> StarknetMessage {
        let call = Call {
            to: *contract_address,
            selector: *entry_point_selector,
            calldata: calldata.clone(),
        };

        StarknetMessage::new(call)
    }
}
