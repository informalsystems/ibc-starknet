use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::client_state::ClientStateQuerier;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::client_id::ClientId;
use crate::types::cosmos::client_state::CometClientState;

pub struct QueryCometClientState;

impl<Chain, Counterparty, Encoding> ClientStateQuerier<Chain, Counterparty>
    for QueryCometClientState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasHeightType
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState>,
    Encoding: CanEncode<ViaCairo, u64>
        + CanDecode<ViaCairo, Vec<Felt>>
        + CanDecode<ViaCairo, CometClientState>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        _height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&client_id.sequence)
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("client_state"), &calldata)
            .await?;

        let raw_client_state: Vec<Felt> = encoding.decode(&output).map_err(Chain::raise_error)?;

        let client_state: CometClientState = encoding
            .decode(&raw_client_state)
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}
