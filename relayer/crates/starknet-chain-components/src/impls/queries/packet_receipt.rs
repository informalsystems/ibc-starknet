use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::queries::packet_receipt::PacketReceiptQuerier;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use hermes_chain_components::traits::types::packets::timeout::HasPacketReceiptType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;

pub struct QueryStarknetPacketReceipt;

impl<Chain, Counterparty> PacketReceiptQuerier<Chain, Counterparty> for QueryStarknetPacketReceipt
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasPacketReceiptType<Counterparty>
        + HasCommitmentProofType
        + HasErrorType,
    Counterparty: HasSequenceType<Chain>,
{
    async fn query_packet_receipt(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
        _sequence: &Counterparty::Sequence,
        _height: &Chain::Height,
    ) -> Result<(Chain::PacketReceipt, Chain::CommitmentProof), Chain::Error> {
        todo!()
    }
}
