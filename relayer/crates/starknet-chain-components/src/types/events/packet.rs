use cgp::prelude::{CanRaiseError, *};
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::types::channel_id::ChannelId;
use crate::types::cosmos::height::Height;
use crate::types::event::{StarknetEvent, UnknownEvent};
use crate::types::messages::ibc::channel::{ChannelOrdering, PortId};
use crate::types::messages::ibc::packet::Sequence;

#[derive(Debug)]
pub enum PacketRelayEvents {
    Send(SendPacketEvent),
    Receive(ReceivePacketEvent),
}

#[derive(Debug)]
pub struct SendPacketEvent {
    pub sequence_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: u64,
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
    pub timeout_timestamp_on_b: u64,
    pub channel_ordering: ChannelOrdering,

    pub packet_data: Vec<Felt>,
}

pub struct DecodePacketRelayEvents;

impl<Encoding, Strategy> Decoder<Encoding, Strategy, PacketRelayEvents> for DecodePacketRelayEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, SendPacketEvent>
        + CanDecode<Strategy, ReceivePacketEvent>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
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
            Ok(PacketRelayEvents::Send(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, SendPacketEvent>
    for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
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
                u64,
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

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ReceivePacketEvent>
    for DecodePacketRelayEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
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
                u64,
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
