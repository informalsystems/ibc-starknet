use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, ConnectionEndQuerier, ConnectionEndQuerierComponent,
    ConnectionEndWithProofsQuerier, ConnectionEndWithProofsQuerierComponent,
    HasCommitmentProofType, HasConnectionEndType, HasConnectionIdType, HasHeightType,
    HasIbcCommitmentPrefix,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_chain_components::types::Secp256k1KeyPair;
use ibc::core::host::types::path::{ConnectionPath, Path};
use ibc_proto::Protobuf;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::connection_id::{ConnectionEnd, ConnectionId};
use crate::types::membership_proof_signer::MembershipVerifierContainer;
use crate::types::status::StarknetChainStatus;

pub struct QueryConnectionEndFromStarknet;

#[cgp_provider(ConnectionEndQuerierComponent)]
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
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<Chain::ConnectionEnd, Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("connection_end"),
                &calldata,
                Some(height),
            )
            .await?;

        encoding.decode(&output).map_err(Chain::raise_error)
    }
}

#[cgp_provider(ConnectionEndWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType<Height = u64>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end_with_proofs(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<(Chain::ConnectionEnd, Chain::CommitmentProof), Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("connection_end"),
                &calldata,
                Some(height),
            )
            .await?;

        let connection_end = encoding.decode(&output).map_err(Chain::raise_error)?;

        let block = chain.query_block(height).await?;

        let unsigned_membership_proof_bytes = MembershipVerifierContainer {
            state_root: block.block_hash.to_bytes_be().to_vec(),
            prefix: chain.ibc_commitment_prefix().clone(),
            path: Path::Connection(ConnectionPath::new(connection_id))
                .to_string()
                .into(),
            value: Some(connection_end.clone().encode_vec()),
        }
        .canonical_bytes();

        let signed_bytes = chain
            .proof_signer()
            .sign(&unsigned_membership_proof_bytes)
            .map_err(Chain::raise_error)?;

        let dummy_proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: signed_bytes,
        };

        Ok((connection_end, dummy_proof))
    }
}
