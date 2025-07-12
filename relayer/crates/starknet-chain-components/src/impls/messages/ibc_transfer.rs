use std::marker::PhantomData;
use std::str::FromStr;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    HasChannelIdType, HasHeightFields, HasMessageType, HasPortIdType, HasTimeoutType,
};
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_core::test_components::chain::traits::{
    HasMemoType, IbcTokenTransferMessageBuilder, IbcTokenTransferMessageBuilderComponent,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::PortId;
use ibc::primitives::Timestamp;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::{StarknetAddress, StarknetMessage};
use crate::traits::{CanCallContract, CanQueryContractAddress, HasBlobType, HasSelectorType};
use crate::types::{ChannelId, Denom, Height, MsgTransfer, PrefixedDenom, StarknetAmount};

pub struct BuildStarknetIbcTransferMessage;

#[cgp_provider(IbcTokenTransferMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildStarknetIbcTransferMessage
where
    Chain: HasAmountType<Amount = StarknetAmount>
        + HasMemoType<Memo = Option<String>>
        + HasMessageType<Message = StarknetMessage>
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
    Counterparty: HasAddressType + HasHeightFields + HasTimeoutType<Timeout = Timestamp>,
    Encoding: CanEncode<ViaCairo, MsgTransfer>
        + CanEncode<ViaCairo, Product![StarknetAddress]>
        + CanEncode<ViaCairo, Product![StarknetAddress, U256]>
        + HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Option<String>>,
{
    async fn build_ibc_token_transfer_messages(
        chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &Counterparty::Address,
        amount: &StarknetAmount,
        memo: &Option<String>,
        timeout_height: Option<&Counterparty::Height>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let encoding = chain.encoding();
        let ics20_contract_address = chain.query_contract_address(PhantomData).await?;

        let approve_message = {
            let calldata = encoding
                .encode(&product![ics20_contract_address, amount.quantity,])
                .map_err(Chain::raise_error)?;

            StarknetMessage::new(amount.token_address.0, selector!("approve"), calldata)
        };

        let transfer_message = {
            let calldata = chain
                .encoding()
                .encode(&product![amount.token_address])
                .map_err(Chain::raise_error)?;

            let denom = {
                let output = chain
                    .call_contract(
                        &ics20_contract_address,
                        &selector!("ibc_token_denom"),
                        &calldata,
                        None,
                    )
                    .await?;

                let prefix_denom_str: Option<String> =
                    encoding.decode(&output).map_err(Chain::raise_error)?;

                match prefix_denom_str {
                    Some(prefix_denom_str) => {
                        PrefixedDenom::from_str(&prefix_denom_str).map_err(Chain::raise_error)?
                    }
                    None => PrefixedDenom {
                        trace_path: vec![],
                        base: Denom::Native(amount.token_address),
                    },
                }
            };

            let timeout_height_on_b = if let Some(timeout_height) = timeout_height {
                Height {
                    revision_number: Counterparty::revision_number(timeout_height),
                    revision_height: Counterparty::revision_height(timeout_height),
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

            let calldata = chain
                .encoding()
                .encode(&ics20_transfer_message)
                .map_err(Chain::raise_error)?;

            StarknetMessage::new(
                ics20_contract_address.0,
                selector!("send_transfer"),
                calldata,
            )
        };

        let messages = vec![approve_message, transfer_message];

        Ok(messages)
    }
}
