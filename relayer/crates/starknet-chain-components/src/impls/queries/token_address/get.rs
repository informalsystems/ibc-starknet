use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_type_components::traits::HasDenomType;
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use poseidon::Poseidon3Hasher;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::StarknetAddress;
use crate::traits::{
    CanCallContract, CanQueryContractAddress, CosmosTokenAddressOnStarknetQuerier,
    CosmosTokenAddressOnStarknetQuerierComponent,
};
use crate::types::PrefixedDenom;

#[cgp_new_provider(CosmosTokenAddressOnStarknetQuerierComponent)]
impl<Chain, Encoding> CosmosTokenAddressOnStarknetQuerier<Chain> for GetCosmosTokenAddressOnStarknet
where
    Chain: HasAsyncErrorType
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasDenomType<Denom = StarknetAddress>
        + CanQueryContractAddress<symbol!("ibc_ics20_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>
        + CanCallContract<Selector = Felt, Blob = Vec<Felt>>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, PrefixedDenom>
        + CanDecode<ViaCairo, Option<StarknetAddress>>,
{
    async fn query_cosmos_token_address_on_starknet(
        chain: &Chain,
        prefixed_denom: &PrefixedDenom,
    ) -> Result<Option<StarknetAddress>, Chain::Error> {
        let encoding = chain.encoding();
        let ics20_contract_address = chain.query_contract_address(PhantomData).await?;

        let denom_serialized = encoding
            .encode(prefixed_denom)
            .map_err(Chain::raise_error)?;

        let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

        let output = chain
            .call_contract(
                &ics20_contract_address,
                &selector!("ibc_token_address"),
                &vec![ibc_prefixed_denom_key],
                None,
            )
            .await?;

        let address = encoding.decode(&output).map_err(Chain::raise_error)?;

        Ok(address)
    }
}
