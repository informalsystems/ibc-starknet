use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::send_message::CanSendSingleMessage;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_test_components::chain::traits::transfer::amount::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc::core::host::types::identifiers::{ChannelId, PortId};
use poseidon::Poseidon3Hasher;
use starknet::core::types::{Call, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::amount::StarknetAmount;
use crate::types::message_response::StarknetMessageResponse;
use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty, Encoding> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertStarknetTokenAddressFromCosmos
where
    Chain: HasAmountType<Amount = StarknetAmount, Denom = StarknetAddress>
        + HasAddressType<Address = StarknetAddress>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + CanCallContract<Selector = Felt, Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanSendSingleMessage<Message = StarknetMessage, MessageResponse = StarknetMessageResponse>
        + CanQueryContractAddress<symbol!("ibc_ics20_contract_address")>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasAmountType<Amount = Amount>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, TracePrefix>
        + CanEncode<ViaCairo, PrefixedDenom>
        + CanEncode<ViaCairo, Denom>
        + CanEncode<ViaCairo, Felt>
        + CanDecode<ViaCairo, Option<StarknetAddress>>
        + CanDecode<ViaCairo, StarknetAddress>,
{
    async fn ibc_transfer_amount_from(
        chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        cosmos_amount: &Amount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<StarknetAmount, Chain::Error> {
        let encoding = chain.encoding();
        let ics20_contract_address = chain.query_contract_address(PhantomData).await?;

        let cosmos_denom = &cosmos_amount.denom;

        let ibc_prefixed_denom = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: port_id.to_string(),
                channel_id: channel_id.to_string(),
            }],
            base: Denom::Hosted(cosmos_denom.to_string()),
        };

        let mut denom_serialized = vec![];

        {
            // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/apps/src/transfer/types.cairo#L120-L130
            for trace_prefix in &ibc_prefixed_denom.trace_path {
                denom_serialized.extend(encoding.encode(trace_prefix).map_err(Chain::raise_error)?);
            }

            denom_serialized.extend(
                encoding
                    .encode(&ibc_prefixed_denom.base)
                    .map_err(Chain::raise_error)?,
            );
        }

        // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/utils/src/utils.cairo#L35
        let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

        let calldata = encoding
            .encode(&ibc_prefixed_denom_key)
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &ics20_contract_address,
                &selector!("ibc_token_address"),
                &calldata,
                None,
            )
            .await?;

        let token_address: Option<StarknetAddress> =
            encoding.decode(&output).map_err(Chain::raise_error)?;

        let token_address = match token_address {
            Some(token_address) => token_address,
            None => {
                let calldata = encoding
                    .encode(&ibc_prefixed_denom)
                    .map_err(Chain::raise_error)?;

                let message = StarknetMessage {
                    call: Call {
                        to: *ics20_contract_address,
                        selector: selector!("create_ibc_token"),
                        calldata,
                    },
                    counterparty_height: None,
                };

                let message_response = chain.send_message(message).await?;

                encoding
                    .decode(&message_response.result)
                    .map_err(Chain::raise_error)?
            }
        };

        Ok(StarknetAmount {
            quantity: cosmos_amount.quantity.into(),
            token_address,
        })
    }

    async fn transmute_counterparty_amount(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        cosmos_amount: &Amount,
        token_address: &StarknetAddress,
    ) -> Result<StarknetAmount, Chain::Error> {
        Ok(StarknetAmount {
            quantity: cosmos_amount.quantity.into(),
            token_address: *token_address,
        })
    }
}
