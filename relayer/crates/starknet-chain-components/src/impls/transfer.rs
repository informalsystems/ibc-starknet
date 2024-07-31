use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::contract::invoke::CanInvokeContract;
use crate::traits::transfer::TokenTransferer;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub const TRANSFER_SELECTOR: Felt = selector!("transfer");

pub struct TransferErc20Token;

impl<Chain> TokenTransferer<Chain> for TransferErc20Token
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = U256>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMethodSelectorType<MethodSelector = Felt>
        + CanInvokeContract,
{
    async fn transfer_token(
        chain: &Chain,
        token_address: &Felt,
        recipient: &Felt,
        amount: &U256,
    ) -> Result<(), Chain::Error> {
        let _tx_hash = chain
            .invoke_contract(
                token_address,
                &TRANSFER_SELECTOR,
                &vec![*recipient, amount.low().into(), amount.high().into()],
            )
            .await?;

        Ok(())
    }
}
