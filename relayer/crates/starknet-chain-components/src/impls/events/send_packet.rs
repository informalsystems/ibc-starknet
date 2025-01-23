use core::marker::PhantomData;

use cgp::prelude::HasAsyncErrorType;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::EventExtractor;
use hermes_chain_components::traits::packet::from_send_packet::PacketFromSendPacketEventBuilder;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::send_packet::{
    HasSendPacketEvent, ProvideSendPacketEvent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::packet::Packet;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, SendPacketEvent};

impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet> + HasEventType,
{
    type SendPacketEvent = SendPacketEvent;
}

impl<Chain, Counterparty> PacketFromSendPacketEventBuilder<Chain, Counterparty>
    for UseStarknetEvents
where
    Chain:
        HasSendPacketEvent<Counterparty> + HasOutgoingPacketType<Counterparty> + HasAsyncErrorType,
{
    async fn build_packet_from_send_packet_event(
        _chain: &Chain,
        _event: &Chain::SendPacketEvent,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
        todo!()
    }
}

impl<Chain, Encoding> EventExtractor<Chain, SendPacketEvent> for UseStarknetEvents
where
    Chain: HasEventType + HasEncoding<AsStarknetEvent, Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Chain::Event> + CanDecode<ViaCairo, Option<PacketRelayEvents>>,
{
    fn try_extract_from_event(
        chain: &Chain,
        _tag: PhantomData<SendPacketEvent>,
        raw_event: &Chain::Event,
    ) -> Option<SendPacketEvent> {
        let event = chain.encoding().decode(raw_event).ok()??;

        match event {
            PacketRelayEvents::Send(event) => Some(event),
            _ => None,
        }
    }
}
