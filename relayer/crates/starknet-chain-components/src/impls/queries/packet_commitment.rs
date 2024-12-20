use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::packet_commitment::PacketCommitmentQuerier;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packets::receive::HasPacketCommitmentType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

pub struct QueryStarknetPacketCommitment;

impl<Chain, Counterparty> PacketCommitmentQuerier<Chain, Counterparty>
    for QueryStarknetPacketCommitment
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + HasPacketCommitmentType<Counterparty>
        + HasCommitmentProofType
        + HasErrorType,
{
    async fn query_packet_commitment(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _sequence: &Chain::Sequence,
        _height: &Chain::Height,
    ) -> Result<(Chain::PacketCommitment, Chain::CommitmentProof), Chain::Error> {
        todo!()
    }
}
