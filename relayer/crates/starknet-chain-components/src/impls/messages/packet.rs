use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_chain_components::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_chain_components::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAckPacketPayloadType;
use hermes_chain_components::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_chain_components::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;

pub struct BuildStarknetPacketMessages;

impl<Chain, Counterparty> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType + HasErrorType,
    Counterparty: HasOutgoingPacketType<Chain> + HasReceivePacketPayloadType<Chain>,
{
    async fn build_receive_packet_message(
        _chain: &Chain,
        _packet: &Counterparty::OutgoingPacket,
        _payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> AckPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType,
    Counterparty: HasAckPacketPayloadType<Chain>,
{
    async fn build_ack_packet_message(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
        _payload: Counterparty::AckPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType,
    Counterparty: HasTimeoutUnorderedPacketPayloadType<Chain>,
{
    async fn build_timeout_unordered_packet_message(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
        _payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
