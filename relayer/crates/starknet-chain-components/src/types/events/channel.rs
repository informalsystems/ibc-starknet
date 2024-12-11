use cgp::prelude::{CanRaiseError, *};
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::types::channel_id::ChannelId;
use crate::types::connection_id::ConnectionId;
use crate::types::event::{StarknetEvent, UnknownEvent};
use crate::types::messages::ibc::channel::{AppVersion, PortId};

#[derive(Debug)]
pub enum ChannelHandshakeEvents {
    Init(ChanOpenInitEvent),
    Ack(ChanOpenAckEvent),
}

#[derive(Debug)]
pub struct ChanOpenInitEvent {
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub connection_id_on_a: ConnectionId,
    pub version_on_a: AppVersion,
}

#[derive(Debug)]
pub struct ChanOpenAckEvent {
    pub port_id_on_a: PortId,
    pub channel_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub channel_id_on_b: ChannelId,
    pub connection_id_on_a: ConnectionId,
}

pub struct DecodeChannelHandshakeEvents;

impl<Encoding, Strategy> Decoder<Encoding, Strategy, ChannelHandshakeEvents>
    for DecodeChannelHandshakeEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, ChanOpenInitEvent>
        + CanDecode<Strategy, ChanOpenAckEvent>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<ChannelHandshakeEvents, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("ChanOpenInitEvent") {
            Ok(ChannelHandshakeEvents::Init(encoding.decode(event)?))
        } else if selector == selector!("ChanOpenAckEvent") {
            Ok(ChannelHandshakeEvents::Ack(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ChanOpenInitEvent>
    for DecodeChannelHandshakeEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![PortId, ChannelId, PortId, ConnectionId, AppVersion]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ChanOpenInitEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            connection_id_on_a,
            version_on_a
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(ChanOpenInitEvent {
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            connection_id_on_a,
            version_on_a,
        })
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ChanOpenAckEvent>
    for DecodeChannelHandshakeEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![PortId, ChannelId, PortId, ChannelId, ConnectionId]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ChanOpenAckEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            connection_id_on_a
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(ChanOpenAckEvent {
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            connection_id_on_a,
        })
    }
}