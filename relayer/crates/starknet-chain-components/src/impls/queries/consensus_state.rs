use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_chain_components::traits::types::consensus_state::HasConsensusStateType;
use hermes_chain_components::traits::types::height::{HasHeightFields, HasHeightType};
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
use crate::types::cosmos::consensus_state::CometConsensusState;
use crate::types::cosmos::height::Height;

pub struct QueryCometConsensusState;

impl<Chain, Counterparty, Encoding> ConsensusStateQuerier<Chain, Counterparty>
    for QueryCometConsensusState
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasHeightType
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
        + CanRaiseError<Encoding::Error>,
    Counterparty:
        HasConsensusStateType<Chain, ConsensusState = CometConsensusState> + HasHeightFields,
    Encoding: CanEncode<ViaCairo, (u64, Height)>
        + CanDecode<ViaCairo, Vec<Felt>>
        + CanDecode<ViaCairo, CometConsensusState>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_consensus_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        _query_height: &Chain::Height, // TODO: figure whether we can perform height specific queries on Starknet
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&(
                client_id.sequence,
                Height {
                    revision_number: Counterparty::revision_number(consensus_height),
                    revision_height: Counterparty::revision_height(consensus_height),
                },
            ))
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("consensus_state"), &calldata)
            .await?;

        let raw_consensus_state: Vec<Felt> =
            encoding.decode(&output).map_err(Chain::raise_error)?;

        let consensus_state: CometConsensusState = encoding
            .decode(&raw_consensus_state)
            .map_err(Chain::raise_error)?;

        Ok(consensus_state)
    }
}
