use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_test_components::chain::traits::transfer::amount::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::impls::types::address::StarknetAddress;
use crate::traits::queries::token_address::CanQueryCosmosTokenAddressOnStarknet;
use crate::types::amount::StarknetAmount;
use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertStarknetTokenAddressFromCosmos
where
    Chain: HasAmountType<Amount = StarknetAmount, Denom = StarknetAddress>
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
