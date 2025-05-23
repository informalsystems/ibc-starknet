use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    HasChannelIdType, HasCommitmentProofType, HasHeightType, HasPacketCommitmentType,
    HasPortIdType, HasSequenceType, PacketIsReceivedQuerier, PacketIsReceivedQuerierComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::{CanCallContract, CanQueryContractAddress, HasBlobType, HasSelectorType};
use crate::types::{ChannelId, PortId as CairoPortId, Sequence, StarknetCommitmentProof};

pub struct QueryPacketIsReceivedOnStarknet;

#[cgp_provider(PacketIsReceivedQuerierComponent)]
impl<Chain, Counterparty, Encoding> PacketIsReceivedQuerier<Chain, Counterparty>
    for QueryPacketIsReceivedOnStarknet
where
    Chain: HasHeightType<Height = u64>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasPacketCommitmentType<Counterparty, PacketCommitment = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,
    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, bool>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_is_received(
        chain: &Chain,
        port_id: &IbcPortId,
        channel_id: &ChannelId,
        sequence: &IbcSequence,
    ) -> Result<bool, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), *sequence])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("packet_receipt"),
                &calldata,
                None,
            )
            .await?;

        let is_received = encoding.decode(&output).map_err(Chain::raise_error)?;

        Ok(is_received)
    }
}
