use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::CanSendSingleMessage;
use hermes_core::chain_type_components::traits::{HasAddressType, HasDenomType};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::traits::queries::token_address::{
    CosmosTokenAddressOnStarknetQuerier, CosmosTokenAddressOnStarknetQuerierComponent,
};
use crate::types::message_response::StarknetMessageResponse;
use crate::types::messages::ibc::denom::PrefixedDenom;

#[cgp_new_provider(CosmosTokenAddressOnStarknetQuerierComponent)]
impl<Chain, Encoding> CosmosTokenAddressOnStarknetQuerier<Chain>
    for CreateCosmosTokenAddressOnStarknet
where
    Chain: HasAsyncErrorType
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasAddressType<Address = StarknetAddress>
        + HasDenomType<Denom = StarknetAddress>
        + CanSendSingleMessage<Message = StarknetMessage, MessageResponse = StarknetMessageResponse>
        + CanQueryContractAddress<symbol!("ibc_ics20_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, PrefixedDenom>
        + CanDecode<ViaCairo, StarknetAddress>,
{
    async fn query_cosmos_token_address_on_starknet(
        chain: &Chain,
        prefixed_denom: &PrefixedDenom,
    ) -> Result<Option<StarknetAddress>, Chain::Error> {
        let encoding = chain.encoding();
        let ics20_contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(prefixed_denom)
            .map_err(Chain::raise_error)?;

        let message = StarknetMessage::new(
            ics20_contract_address.0,
            selector!("create_ibc_token"),
            calldata,
        );

        let message_response = chain.send_message(message).await?;

        let address = encoding
            .decode(&message_response.result)
            .map_err(Chain::raise_error)?;

        Ok(Some(address))
    }
}
