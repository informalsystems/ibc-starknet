use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::accounts::Call;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::messages::transfer::TransferTokenMessageBuilder;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct BuildTransferErc20TokenMessage;

#[derive(Debug, HasField)]
pub struct TransferErc20TokenMessage {
    pub recipient: Felt,
    pub amount: U256,
}

pub type EncodeTransferErc20TokenMessage = CombineEncoders<
    HList![
        EncodeField<symbol!("recipient")>,
        EncodeField<symbol!("amount")>
    ],
>;

impl<Chain, Encoding> TransferTokenMessageBuilder<Chain> for BuildTransferErc20TokenMessage
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + HasMessageType<Message = Call>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, TransferErc20TokenMessage, Encoded = Vec<Felt>>,
{
    fn build_transfer_token_message(
        chain: &Chain,
        recipient: &Felt,
        amount: &StarknetAmount,
    ) -> Result<Call, Chain::Error> {
        let message = TransferErc20TokenMessage {
            recipient: *recipient,
            amount: amount.quantity,
        };

        let calldata = chain
            .encoding()
            .encode(&message)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: amount.token_address,
            selector: TRANSFER_SELECTOR,
            calldata,
        };

        Ok(call)
    }
}
