use core::marker::PhantomData;

use hermes_core::chain_components::traits::{HasChannelIdType, HasPortIdType};
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType, HasDenomType};
use hermes_core::test_components::chain::traits::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use hermes_cosmos_core::test_components::chain::types::Amount;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::impls::StarknetAddress;
use crate::traits::CanQueryCosmosTokenAddressOnStarknet;
use crate::types::{Denom, PrefixedDenom, StarknetAmount, TracePrefix};

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertStarknetTokenAddressFromCosmos
where
    Chain: HasAmountType<Amount = StarknetAmount>
        + HasDenomType<Denom = StarknetAddress>
        + HasAddressType<Address = StarknetAddress>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + CanQueryCosmosTokenAddressOnStarknet
        + CanRaiseAsyncError<String>,
    Counterparty: HasAmountType<Amount = Amount>,
{
    async fn ibc_transfer_amount_from(
        chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        cosmos_amount: &Amount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<StarknetAmount, Chain::Error> {
        let cosmos_denom = &cosmos_amount.denom;

        let ibc_prefixed_denom = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: port_id.to_string(),
                channel_id: channel_id.to_string(),
            }],
            base: Denom::Hosted(cosmos_denom.to_string()),
        };

        let token_address = chain
            .query_cosmos_token_address_on_starknet(&ibc_prefixed_denom)
            .await?
            .ok_or_else(|| {
                Chain::raise_error(format!(
                    "failed to get cosmos token address on starknet: {cosmos_denom}"
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
