use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::token_balance::{TokenBalanceQuerier, TokenBalanceQuerierComponent};
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;

pub struct QueryErc20TokenBalance;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

#[cgp_provider(TokenBalanceQuerierComponent)]
impl<Chain, Encoding> TokenBalanceQuerier<Chain> for QueryErc20TokenBalance
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanCallContract
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanDecode<ViaCairo, U256, Encoded = Vec<Felt>>,
{
    async fn query_token_balance(
        chain: &Chain,
        token_address: &StarknetAddress,
        account_address: &StarknetAddress,
    ) -> Result<StarknetAmount, Chain::Error> {
        let output = chain
            .call_contract(
                token_address,
                &BALANCE_SELECTOR,
                &vec![**account_address],
                None,
            )
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
