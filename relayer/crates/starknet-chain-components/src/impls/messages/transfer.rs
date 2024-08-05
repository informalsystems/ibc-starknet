use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use starknet::accounts::Call;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::messages::transfer::TransferTokenMessageBuilder;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct BuildTransferErc20TokenMessage;

impl<Chain> TransferTokenMessageBuilder<Chain> for BuildTransferErc20TokenMessage
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = U256>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMethodSelectorType<MethodSelector = Felt>
        + HasMessageType<Message = Call>,
{
    fn build_transfer_token_message(
        _chain: &Chain,
        token_address: &Felt,
        recipient: &Felt,
        amount: &U256,
    ) -> Call {
        Call {
            to: *token_address,
            selector: TRANSFER_SELECTOR,
            calldata: vec![*recipient, amount.low().into(), amount.high().into()],
        }
    }
}
