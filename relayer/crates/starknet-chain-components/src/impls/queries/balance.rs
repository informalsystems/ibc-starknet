use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType, HasDenomType};
use hermes_core::encoding_components::traits::{CanDecode, HasEncoding};
use hermes_core::test_components::chain::traits::{BalanceQuerier, BalanceQuerierComponent};
use hermes_prelude::*;
use starknet::core::types::{Felt, U256};

use crate::impls::{StarknetAddress, BALANCE_SELECTOR};
use crate::traits::{CanCallContract, HasBlobType, HasSelectorType};
use crate::types::StarknetAmount;

#[cgp_new_provider(BalanceQuerierComponent)]
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
