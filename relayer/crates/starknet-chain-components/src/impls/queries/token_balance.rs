use cgp_core::error::CanRaiseError;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::token_balance::TokenBalanceQuerier;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub struct QueryErc20TokenBalance;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

impl<Chain> TokenBalanceQuerier<Chain> for QueryErc20TokenBalance
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = U256>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMethodSelectorType<MethodSelector = Felt>
        + CanCallContract
        + CanRaiseError<&'static str>,
{
    async fn query_token_balance(
        chain: &Chain,
        token_address: &Felt,
        account_address: &Felt,
    ) -> Result<U256, Chain::Error> {
        let output = chain
            .call_contract(token_address, &BALANCE_SELECTOR, &vec![*account_address])
            .await?;

        let [e1, e2]: [Felt; 2] = output.try_into().map_err(|_| {
            Chain::raise_error(
                "expect output returned from balance_of query to be consist of two felt252 values",
            )
        })?;

        let low = u128::from_be_bytes(e1.to_bytes_be()[16..].try_into().unwrap());
        let high = u128::from_be_bytes(e2.to_bytes_be()[16..].try_into().unwrap());

        let amount = U256::from_words(low, high);

        Ok(amount)
    }
}
