use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, ChannelEndQuerier, ChannelEndQuerierComponent,
    ChannelEndWithProofsQuerier, ChannelEndWithProofsQuerierComponent, HasChannelEndType,
    HasChannelIdType, HasCommitmentProofType, HasHeightType, HasIbcCommitmentPrefix, HasPortIdType,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_chain_components::types::Secp256k1KeyPair;
use ibc::core::host::types::path::{ChannelEndPath, Path};
use ibc_proto::Protobuf;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::channel_id::{ChannelEnd, ChannelId};
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::membership_proof_signer::MembershipVerifierContainer;
use crate::types::messages::ibc::channel::PortId;
use crate::types::status::StarknetChainStatus;

pub struct QueryChannelEndFromStarknet;

#[cgp_provider(ChannelEndQuerierComponent)]
impl<Chain, Counterparty, Encoding> ChannelEndQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<Chain::ChannelEnd, Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("channel_end"),
                &calldata,
                Some(height),
            )
            .await?;

        encoding.decode(&output).map_err(Chain::raise_error)
    }
}

#[cgp_provider(ChannelEndWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType<Height = u64>
        + CanQueryChainHeight
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end_with_proofs(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<(Chain::ChannelEnd, Chain::CommitmentProof), Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("channel_end"),
                &calldata,
                Some(height),
            )
            .await?;

        let channel_end = encoding.decode(&output).map_err(Chain::raise_error)?;

        let block = chain.query_block(height).await?;

        let unsigned_membership_proof_bytes = MembershipVerifierContainer {
            state_root: block.block_hash.to_bytes_be().to_vec(),
            prefix: chain.ibc_commitment_prefix().clone(),
            path: Path::ChannelEnd(ChannelEndPath::new(port_id, channel_id))
                .to_string()
                .into(),
            value: Some(channel_end.clone().encode_vec()),
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

        Ok((channel_end, dummy_proof))
    }
}
