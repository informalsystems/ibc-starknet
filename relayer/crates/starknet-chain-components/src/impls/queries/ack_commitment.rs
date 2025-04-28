use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasAckCommitmentHashType, HasChannelIdType, HasCommitmentProofType,
    HasHeightType, HasIbcCommitmentPrefix, HasPortIdType, HasSequenceType,
    PacketAckCommitmentQuerier, PacketAckCommitmentQuerierComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_chain_components::types::Secp256k1KeyPair;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use ibc::core::host::types::path::{AckPath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::channel_id::ChannelId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::membership_proof_signer::MembershipVerifierContainer;
use crate::types::messages::ibc::channel::PortId as CairoPortId;
use crate::types::messages::ibc::packet::Sequence;
use crate::types::status::StarknetChainStatus;
pub struct QueryStarknetAckCommitment;

#[cgp_provider(PacketAckCommitmentQuerierComponent)]
impl<Chain, Counterparty, Encoding> PacketAckCommitmentQuerier<Chain, Counterparty>
    for QueryStarknetAckCommitment
where
    Chain: HasHeightType<Height = u64>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasAckCommitmentHashType<AckCommitmentHash = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,

    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, Product![[u32; 8]]>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_ack_commitment_with_proof(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        sequence: &IbcSequence,
        height: &u64,
    ) -> Result<(Vec<u8>, StarknetCommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), *sequence])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("packet_acknowledgement"),
                &calldata,
                Some(height),
            )
            .await?;

        let product![ack,] = encoding.decode(&output).map_err(Chain::raise_error)?;

        let ack_bytes = ack
            .into_iter()
            // TODO(rano): cairo uses [u32; 8], but in cosmos it's Vec<u8>
            .flat_map(|felt| felt.to_be_bytes())
            .collect::<Vec<_>>();

        let block = chain.query_block(height).await?;

        let unsigned_membership_proof_bytes = MembershipVerifierContainer {
            state_root: block.block_hash.to_bytes_be().to_vec(),
            prefix: chain.ibc_commitment_prefix().clone(),
            path: Path::Ack(AckPath::new(port_id, channel_id, *sequence))
                .to_string()
                .into(),
            value: Some(ack_bytes.clone()),
        }
        .canonical_bytes();

        let signed_bytes = chain
            .proof_signer()
            .sign(&unsigned_membership_proof_bytes)
            .map_err(Chain::raise_error)?;

        // TODO(rano): how to get the proof?
        let dummy_proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: signed_bytes,
        };

        // `ack_bytes` is stored after hashing.
        // query block event is required to get the original ack_bytes.

        Ok((ack_bytes, dummy_proof))
    }
}
