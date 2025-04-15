use core::marker::PhantomData;
use std::string::FromUtf8Error;

use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_cosmos_test_components::chain::impls::transfer::amount::derive_ibc_denom;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_test_components::chain::traits::transfer::amount::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::types::amount::StarknetAmount;

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertCosmosIbcAmountFromStarknet
where
    Chain: HasAmountType<Amount = Amount>
        + HasDenomType<Denom = Denom>
        + CanRaiseAsyncError<FromUtf8Error>
        + CanRaiseAsyncError<String>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>,
    Counterparty: HasAmountType<Amount = StarknetAmount>,
{
    async fn ibc_transfer_amount_from(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &StarknetAmount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<Amount, Chain::Error> {
        let denom = derive_ibc_denom(
            port_id,
            channel_id,
            &Denom::Base(counterparty_amount.token_address.to_string()),
        )
        .map_err(Chain::raise_error)?;

        let counterparty_quantity = counterparty_amount.quantity;

        if counterparty_quantity.high() > 0 {
            return Err(Chain::raise_error(format!(
                "cannot convert U256 amount to u128: {counterparty_quantity}"
            )));
        }

        let quantity = counterparty_quantity.low();

        Ok(Amount { quantity, denom })
    }

    async fn transmute_counterparty_amount(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &StarknetAmount,
        denom: &Denom,
    ) -> Result<Amount, Chain::Error> {
        let counterparty_quantity = counterparty_amount.quantity;

        if counterparty_quantity.high() > 0 {
            return Err(Chain::raise_error(format!(
                "cannot convert U256 amount to u128: {counterparty_quantity}"
            )));
        }

        let quantity = counterparty_quantity.low();

        Ok(Amount {
            quantity,
            denom: denom.clone(),
        })
    }
}
