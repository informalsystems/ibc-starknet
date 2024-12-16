use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionEndWithProofsQuerier,
};
use hermes_chain_components::traits::types::connection::HasConnectionEndType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::HasConnectionIdType;
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
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::connection_id::{ConnectionEnd, ConnectionId};

pub struct QueryConnectionEndFromStarknet;

impl<Chain, Counterparty, Encoding> ConnectionEndQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        _height: &Chain::Height,
    ) -> Result<Chain::ConnectionEnd, Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("connection_end"), &calldata)
            .await?;

        Ok(encoding.decode(&output).map_err(Chain::raise_error)?)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end_with_proofs(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        _height: &Chain::Height,
    ) -> Result<(Chain::ConnectionEnd, Chain::CommitmentProof), Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("connection_end"), &calldata)
            .await?;

        // TODO(rano): how to get the proof?
        let dummy_proof = StarknetCommitmentProof {
            proof_height: 0,
            proof_bytes: vec![],
        };

        Ok((
            encoding.decode(&output).map_err(Chain::raise_error)?,
            dummy_proof,
        ))
    }
}
