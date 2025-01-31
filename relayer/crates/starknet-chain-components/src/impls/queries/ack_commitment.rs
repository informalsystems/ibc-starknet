use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerier;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::channel_id::ChannelId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::messages::ibc::channel::PortId as CairoPortId;
use crate::types::messages::ibc::packet::Sequence;

pub struct QueryStarknetAckCommitment;

impl<Chain, Counterparty, Encoding> PacketAcknowledgementQuerier<Chain, Counterparty>
    for QueryStarknetAckCommitment
where
    Chain: HasHeightType<Height = u64>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,

    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, Product![[u32; 8]]>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_acknowledgement(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        sequence: &IbcSequence,
        height: &u64,
    ) -> Result<(Vec<u8>, StarknetCommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let cairo_sequence = Sequence {
            sequence: sequence.value(),
        };

        let calldata = encoding
            .encode(&product![
                port_id.clone(),
                channel_id.clone(),
                cairo_sequence
            ])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("packet_acknowledgement"),
                &calldata,
            )
            .await?;

        // TODO(rano): how to get the proof?
        let dummy_proof = StarknetCommitmentProof {
            proof_height: *height,
            proof_bytes: vec![0x1],
        };

        let product![ack,] = encoding.decode(&output).map_err(Chain::raise_error)?;

        let ack_bytes = ack
            .into_iter()
            // TODO(rano): cairo uses [u32; 8], but in cosmos it's Vec<u8>
            .flat_map(|felt| felt.to_be_bytes())
            .collect::<Vec<_>>();

        Ok((ack_bytes, dummy_proof))
    }
}
