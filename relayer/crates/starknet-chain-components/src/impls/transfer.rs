use cgp::prelude::*;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::core::types::Felt;
use starknet::macros::selector;

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
