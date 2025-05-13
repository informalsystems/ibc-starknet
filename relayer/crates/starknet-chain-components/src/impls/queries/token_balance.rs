use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::encoding_components::traits::{CanDecode, HasEncoding};
use hermes_prelude::*;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::StarknetAddress;
use crate::traits::{
    CanCallContract, HasBlobType, HasSelectorType, TokenBalanceQuerier,
    TokenBalanceQuerierComponent,
};
use crate::types::StarknetAmount;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

#[cgp_new_provider(TokenBalanceQuerierComponent)]
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
