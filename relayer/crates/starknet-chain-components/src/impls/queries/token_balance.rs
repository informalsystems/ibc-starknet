use cgp_core::error::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::token_balance::TokenBalanceQuerier;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;
use crate::types::amount::StarknetAmount;

pub struct QueryErc20TokenBalance;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

impl<Chain, Encoding> TokenBalanceQuerier<Chain> for QueryErc20TokenBalance
where
    Chain: HasAddressType<Address = Felt>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasMethodSelectorType<MethodSelector = Felt>
        + CanCallContract
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<&'static str>
        + CanRaiseError<Encoding::Error>,
    Encoding: CanDecode<ViaCairo, U256, Encoded = Vec<Felt>>,
{
    async fn query_token_balance(
        chain: &Chain,
        token_address: &Felt,
        account_address: &Felt,
    ) -> Result<StarknetAmount, Chain::Error> {
        let output = chain
            .call_contract(token_address, &BALANCE_SELECTOR, &vec![*account_address])
            .await?;

        let quantity = chain
            .encoding()
            .decode(&output)
            .map_err(Chain::raise_error)?;

        Ok(StarknetAmount {
            quantity,
            token_address: *token_address,
        })
    }
}
