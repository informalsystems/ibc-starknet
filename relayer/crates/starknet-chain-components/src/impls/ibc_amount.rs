use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
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
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::amount::StarknetAmount;
use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty, Encoding> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertStarknetTokenAddressFromCosmos
where
    Chain: HasAmountType<Amount = StarknetAmount, Denom = StarknetAddress>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + CanCallContract<Selector = Felt, Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_ics20_contract_address")>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasAmountType<Amount = Amount>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, TracePrefix>
        + CanEncode<ViaCairo, Denom>
        + CanEncode<ViaCairo, Felt>
        + CanDecode<ViaCairo, Option<StarknetAddress>>,
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

        let token_address = token_address.ok_or_else(|| {
            Chain::raise_error(format!(
                "token address not found for Cosmos denom: {cosmos_denom}"
            ))
        })?;

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
