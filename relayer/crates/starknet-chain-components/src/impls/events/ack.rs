use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::{EventExtractor, EventExtractorComponent};
use hermes_chain_components::traits::packet::from_write_ack::{
    PacketFromWriteAckEventBuilder, PacketFromWriteAckEventBuilderComponent,
};
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::{
    HasWriteAckEvent, ProvideWriteAckEvent, WriteAckEventComponent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::host::types::error::IdentifierError;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, WriteAcknowledgementEvent};

#[cgp_provider(WriteAckEventComponent)]
impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAcknowledgementEvent;
}

#[cgp_provider(EventExtractorComponent)]
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

#[cgp_provider(PacketFromWriteAckEventBuilderComponent)]
impl<Chain, Counterparty> PacketFromWriteAckEventBuilder<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgementEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<ChannelError>
        + CanRaiseAsyncError<serde_json::Error>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
{
    async fn build_packet_from_write_ack_event(
        _chain: &Chain,
        event: &WriteAcknowledgementEvent,
    ) -> Result<Packet, Chain::Error> {
        let packet = Packet {
            seq_on_a: event.sequence_on_a,
            port_id_on_a: event.port_id_on_a.clone(),
            chan_id_on_a: event.channel_id_on_a.clone(),
            port_id_on_b: event.port_id_on_b.clone(),
            chan_id_on_b: event.channel_id_on_b.clone(),
            // FIXME: make the Cairo contract include these fields in the event
            data: Vec::new(),
            timeout_height_on_b: TimeoutHeight::Never,
            timeout_timestamp_on_b: TimeoutTimestamp::Never,
        };

        Ok(packet)
    }

    async fn build_ack_from_write_ack_event(
        _chain: &Chain,
        ack: &WriteAcknowledgementEvent,
    ) -> Result<Vec<u8>, Chain::Error> {
        // FIXME: Fix the Cairo contract to return ByteArray acknowledgement inside event.
        // The Cairo encoding for ByteArray is different from Array<u8>

        let ack_bytes = ack
            .acknowledgement
            .ack
            .iter()
            .map(|&felt| felt.try_into().unwrap())
            .collect::<Vec<_>>();

        Ok(ack_bytes)
    }
}
