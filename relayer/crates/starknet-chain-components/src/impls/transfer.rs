use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::messages::transfer::CanBuildTransferTokenMessage;
use crate::traits::transfer::TokenTransferer;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct TransferErc20Token;

impl<Chain> TokenTransferer<Chain> for TransferErc20Token
where
    Chain: HasAddressType
        + HasAmountType
        + HasBlobType
        + HasMethodSelectorType
        + CanBuildTransferTokenMessage
        + CanSendSingleMessage,
{
    async fn transfer_token(
        chain: &Chain,
        token_address: &Chain::Address,
        recipient: &Chain::Address,
        amount: &Chain::Amount,
    ) -> Result<(), Chain::Error> {
        let message = chain.build_transfer_token_message(token_address, recipient, amount);

        chain.send_message(message).await?;

        Ok(())
    }
}
