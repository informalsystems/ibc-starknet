use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::channel_end::{
    ChannelEndQuerier, ChannelEndWithProofsQuerier,
};
use hermes_chain_components::traits::types::channel::HasChannelEndType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
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
use crate::types::channel_id::{ChannelEnd, ChannelId};
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::messages::ibc::channel::PortId;

pub struct QueryChannelEndFromStarknet;

impl<Chain, Counterparty, Encoding> ChannelEndQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        _height: &Chain::Height,
    ) -> Result<Chain::ChannelEnd, Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let port_id = PortId {
            port_id: port_id.to_string(),
        };

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("channel_end"), &calldata)
            .await?;

        encoding.decode(&output).map_err(Chain::raise_error)
    }
}

impl<Chain, Counterparty, Encoding> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end_with_proofs(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        _height: &Chain::Height,
    ) -> Result<(Chain::ChannelEnd, Chain::CommitmentProof), Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let port_id = PortId {
            port_id: port_id.to_string(),
        };

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("channel_end"), &calldata)
            .await?;

        // TODO(rano): how to get the proof?
        let dummy_proof = StarknetCommitmentProof {
            proof_height: 0,
            proof_bytes: vec![0x1],
        };

        Ok((
            encoding.decode(&output).map_err(Chain::raise_error)?,
            dummy_proof,
        ))
    }
}
