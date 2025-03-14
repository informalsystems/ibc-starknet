use std::marker::PhantomData;
use std::str::FromStr;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_test_components::chain::traits::messages::ibc_transfer::{
    IbcTokenTransferMessageBuilder, IbcTokenTransferMessageBuilderComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc::core::host::types::identifiers::PortId;
use ibc::primitives::Timestamp;
use starknet::core::types::{Call, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;
use crate::types::channel_id::ChannelId;
use crate::types::cosmos::height::Height;
use crate::types::messages::ibc::denom::PrefixedDenom;
use crate::types::messages::ibc::ibc_transfer::MsgTransfer;

pub struct BuildStarknetIbcTransferMessage;

#[cgp_provider(IbcTokenTransferMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildStarknetIbcTransferMessage
where
    Chain: HasAsyncErrorType
        + HasAmountType<Amount = StarknetAmount>
        + HasMemoType<Memo = Option<String>>
        + HasMessageType<Message = StarknetMessage>
        + HasHeightType<Height = u64>
        + HasTimeoutType<Timeout = Timestamp>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasAddressType<Address = StarknetAddress>
        + CanCallContract
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_ics20_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<String>,
    Counterparty: HasAddressType,
    Encoding: CanEncode<ViaCairo, MsgTransfer>
        + CanEncode<ViaCairo, Product![StarknetAddress]>
        + HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, String>,
{
    async fn build_ibc_token_transfer_message(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &Counterparty::Address,
        amount: &StarknetAmount,
        memo: &Option<String>,
        timeout_height: Option<&u64>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<Chain::Message, Chain::Error> {
        let ics20_contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&product![amount.token_address])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &ics20_contract_address,
                &selector!("ibc_token_denom"),
                &calldata,
                None,
            )
            .await?;

        let prefix_denom_str = chain
            .encoding()
            .decode(&output)
            .map_err(Chain::raise_error)?;

        let denom = PrefixedDenom::from_str(&prefix_denom_str).map_err(Chain::raise_error)?;

        let timeout_height_on_b = if let Some(timeout_height) = timeout_height {
            Height {
                revision_number: 0,
                revision_height: *timeout_height,
            }
        } else {
            Height {
                revision_number: 0,
                revision_height: 0,
            }
        };

        let timeout_timestamp_on_b = if let Some(timeout_time) = timeout_time {
            *timeout_time
        } else {
            Timestamp::from_nanoseconds(0)
        };

        let memo = if let Some(memo) = memo {
            memo.clone()
        } else {
            String::new()
        };

        let ics20_transfer_message = MsgTransfer {
            port_id_on_a: port_id.clone(),
            chan_id_on_a: channel_id.clone(),
            denom,
            amount: amount.quantity,
            receiver: recipient_address.to_string(),
            memo,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        let call_data = chain
            .encoding()
            .encode(&ics20_transfer_message)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: ics20_contract_address.0,
            selector: selector!("send_transfer"),
            calldata: call_data,
        };

        Ok(StarknetMessage::new(call))
    }
}
