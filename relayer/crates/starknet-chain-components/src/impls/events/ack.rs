use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::EventExtractor;
use hermes_chain_components::traits::packet::from_write_ack::PacketFromWriteAckBuilder;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::{
    HasWriteAckEvent, ProvideWriteAckEvent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, WriteAcknowledgementEvent};

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAcknowledgementEvent;

    fn write_acknowledgement(ack: &WriteAcknowledgementEvent) -> impl AsRef<Vec<u8>> + Send {
        ack.acknowledgement
            .ack
            .iter()
            .map(|&felt| felt.try_into().unwrap())
            .collect::<Vec<_>>()
    }
}

impl<Chain, Encoding> EventExtractor<Chain, WriteAcknowledgementEvent> for UseStarknetEvents
where
    Chain: HasEventType + HasEncoding<AsStarknetEvent, Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Chain::Event> + CanDecode<ViaCairo, Option<PacketRelayEvents>>,
{
    fn try_extract_from_event(
        chain: &Chain,
        _tag: PhantomData<WriteAcknowledgementEvent>,
        raw_event: &Chain::Event,
    ) -> Option<WriteAcknowledgementEvent> {
        let event = chain.encoding().decode(raw_event).ok()??;

        match event {
            PacketRelayEvents::WriteAcknowledgement(ack) => Some(ack),
            _ => None,
        }
    }
}

impl<Chain, Counterparty> PacketFromWriteAckBuilder<Chain, Counterparty> for UseStarknetEvents
where
    Chain: Sized + HasWriteAckEvent<Counterparty>,
    Counterparty: HasOutgoingPacketType<Chain>,
{
    fn build_packet_from_write_ack_event(
        _ack: &Chain::WriteAckEvent,
    ) -> &Counterparty::OutgoingPacket {
        todo!()
    }
}
