use core::marker::PhantomData;
use std::string::FromUtf8Error;

use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_test_components::chain::traits::transfer::amount::{
    IbcTransferredAmountConverter, IbcTransferredAmountConverterComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::impls::types::address::StarknetAddress;
use crate::types::amount::StarknetAmount;

#[cgp_new_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty> IbcTransferredAmountConverter<Chain, Counterparty>
    for ConvertStarknetTokenAddressFromCosmos
where
    Chain: HasAmountType<Amount = StarknetAmount, Denom = StarknetAddress>
        + CanRaiseAsyncError<String>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>,
    Counterparty: HasAmountType<Amount = Amount>,
{
    async fn ibc_transfer_amount_from(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Amount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<StarknetAmount, Chain::Error> {
        todo!()
    }

    async fn transmute_counterparty_amount(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Amount,
        token_address: &StarknetAddress,
    ) -> Result<StarknetAmount, Chain::Error> {
        todo!()
    }
}
