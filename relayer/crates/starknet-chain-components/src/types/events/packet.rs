use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::encoding_components::traits::{
    CanDecode, Decoder, DecoderComponent, HasEncodedType, HasEncoding,
};
use hermes_prelude::*;
use ibc::core::channel::types::channel::Order as ChannelOrdering;
use ibc::core::host::types::identifiers::ClientId;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::StarknetUpdateClientEvent;
use crate::types::{
    Acknowledgement, ChannelId, Height, Packet, PortId, Sequence, StarknetEvent, Timestamp,
    UnknownEvent,
};

#[derive(Debug)]
pub enum PacketRelayEvents {
    Send(SendPacketEvent),
    Receive(ReceivePacketEvent),
    WriteAcknowledgement(WriteAcknowledgementEvent),
    Acknowledge(AcknowledgePacketEvent),
    Timeout(TimeoutPacketEvent),
    UpdateClient(StarknetUpdateClientEvent),
}

#[derive(Debug)]
pub struct SendPacketEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
    pub channel_ordering: ChannelOrdering,

    pub packet_data: Vec<Felt>,
}

#[derive(Debug)]
pub struct ReceivePacketEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
    pub channel_ordering: ChannelOrdering,

    pub packet_data: Vec<Felt>,
}

#[derive(Debug)]
pub struct WriteAcknowledgementEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,

    pub packet: Packet,
    pub acknowledgement: Acknowledgement,
}

#[derive(Debug)]
pub struct AcknowledgePacketEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
    pub channel_ordering: ChannelOrdering,
}

#[derive(Debug)]
pub struct TimeoutPacketEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
    pub channel_ordering: ChannelOrdering,
}

pub struct DecodePacketRelayEvents;

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy> Decoder<Encoding, Strategy, PacketRelayEvents> for DecodePacketRelayEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, SendPacketEvent>
        + CanDecode<Strategy, ReceivePacketEvent>
        + CanDecode<Strategy, WriteAcknowledgementEvent>
        + CanDecode<Strategy, AcknowledgePacketEvent>
        + CanDecode<Strategy, TimeoutPacketEvent>
        + CanDecode<Strategy, StarknetUpdateClientEvent>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<PacketRelayEvents, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("SendPacketEvent") {
            Ok(PacketRelayEvents::Send(encoding.decode(event)?))
        } else if selector == selector!("ReceivePacketEvent") {
            Ok(PacketRelayEvents::Receive(encoding.decode(event)?))
        } else if selector == selector!("WriteAcknowledgementEvent") {
            Ok(PacketRelayEvents::WriteAcknowledgement(
                encoding.decode(event)?,
            ))
        } else if selector == selector!("AcknowledgePacketEvent") {
            Ok(PacketRelayEvents::Acknowledge(encoding.decode(event)?))
        } else if selector == selector!("TimeoutPacketEvent") {
            Ok(PacketRelayEvents::Timeout(encoding.decode(event)?))
        } else if selector == selector!("UpdateClientEvent") {
            Ok(PacketRelayEvents::UpdateClient(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, SendPacketEvent>
    for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<
            ViaCairo,
            Product![
                Sequence,
                PortId,
                ChannelId,
                PortId,
                ChannelId,
                Height,
                Timestamp,
                ChannelOrdering
            ],
        > + CanDecode<ViaCairo, Vec<Felt>>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<SendPacketEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let packet_data = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(SendPacketEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
            packet_data,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ReceivePacketEvent>
    for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<
            ViaCairo,
            Product![
                Sequence,
                PortId,
                ChannelId,
                PortId,
                ChannelId,
                Height,
                Timestamp,
                ChannelOrdering
            ],
        > + CanDecode<ViaCairo, Vec<Felt>>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ReceivePacketEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let packet_data = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(ReceivePacketEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
            packet_data,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, WriteAcknowledgementEvent> for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Sequence, PortId, ChannelId, PortId, ChannelId]>
        + CanDecode<ViaCairo, Product![Packet, Acknowledgement]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<WriteAcknowledgementEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![packet, acknowledgement,] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(WriteAcknowledgementEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            packet,
            acknowledgement,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, AcknowledgePacketEvent> for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<
            ViaCairo,
            Product![
                Sequence,
                PortId,
                ChannelId,
                PortId,
                ChannelId,
                Height,
                Timestamp,
                ChannelOrdering
            ],
        >,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<AcknowledgePacketEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        // Only asserting non-empty as there is no data field in Cairo
        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(AcknowledgePacketEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, TimeoutPacketEvent>
    for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<
            ViaCairo,
            Product![
                Sequence,
                PortId,
                ChannelId,
                PortId,
                ChannelId,
                Height,
                Timestamp,
                ChannelOrdering
            ],
        >,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<TimeoutPacketEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        // Only asserting non-empty as there is no data field in Cairo
        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(TimeoutPacketEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            timeout_height_on_b,
            timeout_timestamp_on_b,
            channel_ordering,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, StarknetUpdateClientEvent> for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![ClientId]>
        + CanDecode<ViaCairo, Product![Vec<Height>, Vec<Felt>]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<StarknetUpdateClientEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![client_id,] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![consensus_heights, header,] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(StarknetUpdateClientEvent {
            client_id,
            consensus_heights,
            header,
        })
    }
}
