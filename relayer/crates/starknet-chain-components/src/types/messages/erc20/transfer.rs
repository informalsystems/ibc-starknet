use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::messages::transfer::{
    TransferTokenMessageBuilder, TransferTokenMessageBuilderComponent,
};
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;

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
