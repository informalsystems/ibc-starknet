use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerier;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

pub struct QueryStarknetAckCommitment;

impl<Chain, Counterparty> PacketAcknowledgementQuerier<Chain, Counterparty>
    for QueryStarknetAckCommitment
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasAcknowledgementType<Counterparty>
        + HasCommitmentProofType
        + HasErrorType,
    Counterparty: HasSequenceType<Chain>,
{
    async fn query_packet_acknowledgement(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _sequence: &Counterparty::Sequence,
        _height: &Chain::Height,
    ) -> Result<(Chain::Acknowledgement, Chain::CommitmentProof), Chain::Error> {
        todo!()
    }
}
