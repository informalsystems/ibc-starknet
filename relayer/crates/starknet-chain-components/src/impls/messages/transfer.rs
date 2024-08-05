use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::messages::transfer::TransferTokenMessageBuilder;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;
use crate::types::amount::StarknetAmount;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct BuildTransferErc20TokenMessage;

impl<Chain> TransferTokenMessageBuilder<Chain> for BuildTransferErc20TokenMessage
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMethodSelectorType<MethodSelector = Felt>
        + HasMessageType<Message = Call>,
{
    fn build_transfer_token_message(
        _chain: &Chain,
        recipient: &Felt,
        amount: &StarknetAmount,
    ) -> Call {
        let quantity = amount.quantity;

        Call {
            to: amount.token_address,
            selector: TRANSFER_SELECTOR,
            calldata: vec![*recipient, quantity.low().into(), quantity.high().into()],
        }
    }
}
