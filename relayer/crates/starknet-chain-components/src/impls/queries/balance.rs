use cgp::prelude::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::amount::StarknetAmount;

pub const BALANCE_SELECTOR: Felt = selector!("balance_of");

pub struct QueryStarknetWalletBalance;

impl<Chain, Encoding> BalanceQuerier<Chain> for QueryStarknetWalletBalance
where
    Chain: HasAddressType<Address = Felt>
        + HasDenomType<Denom = Felt>
        + HasAmountType<Amount = StarknetAmount>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanCallContract
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseError<&'static str>
        + CanRaiseError<Encoding::Error>,
    Encoding: CanDecode<ViaCairo, U256, Encoded = Vec<Felt>>,
{
    async fn query_balance(
        chain: &Chain,
        address: &Chain::Address,
        denom: &Chain::Denom,
    ) -> Result<Chain::Amount, Chain::Error> {
        let output = chain
            .call_contract(denom, &BALANCE_SELECTOR, &vec![*address])
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
