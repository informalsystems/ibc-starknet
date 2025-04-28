use core::time::Duration;

use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CanSendSingleMessage, HasHeightType, HasTimeType, HasTimeoutType,
};
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::test_components::chain::traits::{
    IbcTransferTimeoutCalculator, IbcTransferTimeoutCalculatorComponent,
};
use hermes_cosmos_core::chain_components::types::Time;
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::macros::selector;
use time::OffsetDateTime;

use crate::traits::messages::transfer::CanBuildTransferTokenMessage;
use crate::traits::transfer::{TokenTransferComponent, TokenTransferer};
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct TransferErc20Token;

#[cgp_provider(TokenTransferComponent)]
impl<Chain> TokenTransferer<Chain> for TransferErc20Token
where
    Chain: HasAddressType
        + HasAmountType
        + HasBlobType
        + HasSelectorType
        + CanBuildTransferTokenMessage
        + CanSendSingleMessage,
{
    async fn transfer_token(
        chain: &Chain,
        recipient: &Chain::Address,
        amount: &Chain::Amount,
    ) -> Result<(), Chain::Error> {
        let message = chain.build_transfer_token_message(recipient, amount)?;

        chain.send_message(message).await?;

        Ok(())
    }
}

pub struct IbcTransferTimeoutAfterSeconds<const SECS: u64>;

#[cgp_provider(IbcTransferTimeoutCalculatorComponent)]
impl<Chain, Counterparty, const SECS: u64> IbcTransferTimeoutCalculator<Chain, Counterparty>
    for IbcTransferTimeoutAfterSeconds<SECS>
where
    Counterparty: HasTimeType<Time = Time> + HasTimeoutType<Timeout = Timestamp> + HasHeightType,
{
    fn ibc_transfer_timeout_time(_chain: &Chain, current_time: &Time) -> Option<Timestamp> {
        let time = (*current_time + Duration::from_secs(SECS)).unwrap();
        OffsetDateTime::from(time).try_into().ok()
    }

    fn ibc_transfer_timeout_height(
        _chain: &Chain,
        _current_height: &Counterparty::Height,
    ) -> Option<Counterparty::Height> {
        None
    }
}
