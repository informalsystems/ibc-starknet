use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::commitment_prefix::HasIbcCommitmentPrefix;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_chain_components::traits::queries::packet_receipt::{
    PacketReceiptQuerier, PacketReceiptQuerierComponent,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packets::timeout::HasPacketReceiptType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use ibc::core::host::types::path::{Path, ReceiptPath};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::channel_id::ChannelId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::membership_proof_signer::MembershipVerifierContainer;
use crate::types::messages::ibc::channel::PortId as CairoPortId;
use crate::types::messages::ibc::packet::Sequence;
use crate::types::status::StarknetChainStatus;

pub struct QueryStarknetPacketReceipt;

#[cgp_provider(PacketReceiptQuerierComponent)]
impl<Chain, Counterparty, Encoding> PacketReceiptQuerier<Chain, Counterparty>
    for QueryStarknetPacketReceipt
where
    Chain: HasHeightType<Height = u64>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasPacketReceiptType<Counterparty, PacketReceipt = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,
    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, bool>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_receipt(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        sequence: &IbcSequence,
        _height: &u64,
    ) -> Result<(Option<Vec<u8>>, StarknetCommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), *sequence])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(&contract_address, &selector!("packet_receipt"), &calldata)
            .await?;

        let receipt_status = encoding.decode(&output).map_err(Chain::raise_error)?;

        let receipt = if receipt_status { Some(vec![1]) } else { None };

        let chain_status = chain.query_chain_status().await?;

        let unsigned_membership_proof_bytes = MembershipVerifierContainer {
            state_root: chain_status.block_hash.to_bytes_be().to_vec(),
            prefix: chain.ibc_commitment_prefix().clone(),
            path: Path::Receipt(ReceiptPath::new(port_id, channel_id, *sequence))
                .to_string()
                .into(),
            value: receipt.clone(),
        }
        .canonical_bytes();

        let signed_bytes = chain
            .proof_signer()
            .sign(&unsigned_membership_proof_bytes)
            .map_err(Chain::raise_error)?;

        let dummy_proof = StarknetCommitmentProof {
            proof_height: chain_status.height,
            proof_bytes: signed_bytes,
        };

        Ok((receipt, dummy_proof))
    }
}
