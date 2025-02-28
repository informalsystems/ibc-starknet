use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_test_components::chain::traits::queries::balance::{
    BalanceQuerier, BalanceQuerierComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::traits::contract::call::CanCallContract;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

pub struct QueryStarknetWalletBalance;

#[cgp_provider(BalanceQuerierComponent)]
impl<Chain, Encoding> BalanceQuerier<Chain> for QueryStarknetWalletBalance
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasDenomType<Denom = StarknetAddress>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanCallContract
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanDecode<ViaCairo, U256, Encoded = Vec<Felt>>,
{
    async fn query_balance(
        chain: &Chain,
        address: &Chain::Address,
        denom: &Chain::Denom,
    ) -> Result<Chain::Amount, Chain::Error> {
        let output = chain
            .call_contract(denom, &BALANCE_SELECTOR, &vec![**address], None)
            .await?;

        let quantity = chain
            .encoding()
            .decode(&output)
            .map_err(Chain::raise_error)?;

        Ok(StarknetAmount {
            quantity,
            token_address: *denom,
        })
    }
}
