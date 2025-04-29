use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::HasMessageType;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::encoding_components::traits::{CanEncode, HasEncoding};
use hermes_prelude::*;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::{StarknetAddress, StarknetMessage};
use crate::traits::{
    HasBlobType, HasSelectorType, TransferTokenMessageBuilder, TransferTokenMessageBuilderComponent,
};
use crate::types::StarknetAmount;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

#[derive(Debug, HasField, HasFields)]
pub struct TransferErc20TokenMessage {
    pub recipient: StarknetAddress,
    pub amount: U256,
}

#[cgp_new_provider(TransferTokenMessageBuilderComponent)]
impl<Chain, Encoding> TransferTokenMessageBuilder<Chain> for BuildTransferErc20TokenMessage
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, TransferErc20TokenMessage, Encoded = Vec<Felt>>,
{
    fn build_transfer_token_message(
        chain: &Chain,
        recipient: &StarknetAddress,
        amount: &StarknetAmount,
    ) -> Result<StarknetMessage, Chain::Error> {
        let message = TransferErc20TokenMessage {
            recipient: *recipient,
            amount: amount.quantity,
        };

        let calldata = chain
            .encoding()
            .encode(&message)
            .map_err(Chain::raise_error)?;

        let message = StarknetMessage::new(*amount.token_address, TRANSFER_SELECTOR, calldata);

        Ok(message)
    }
}
