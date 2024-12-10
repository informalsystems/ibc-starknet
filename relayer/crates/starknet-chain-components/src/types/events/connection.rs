use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::types::client_id::ClientId;
use crate::types::connection_id::ConnectionId;
use crate::types::event::{StarknetEvent, UnknownEvent};

#[derive(Debug)]
pub enum ConnectionHandshakeEvents {
    Init(InitConnectionEvent),
    Ack(ConnOpenAckEvent),
}

#[derive(Debug)]
pub struct InitConnectionEvent {
    pub client_id_on_a: ClientId,
    pub connection_id_on_a: ConnectionId,
    pub client_id_on_b: ClientId,
}

#[derive(Debug)]
pub struct ConnOpenAckEvent {
    pub client_id_on_a: ClientId,
    pub connection_id_on_a: ConnectionId,
    pub client_id_on_b: ClientId,
    pub connection_id_on_b: ConnectionId,
}

pub struct DecodeConnectionHandshakeEvents;

impl<Encoding, Strategy> Decoder<Encoding, Strategy, ConnectionHandshakeEvents>
    for DecodeConnectionHandshakeEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, InitConnectionEvent>
        + CanDecode<Strategy, ConnOpenAckEvent>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<ConnectionHandshakeEvents, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("ConnOpenInitEvent") {
            Ok(ConnectionHandshakeEvents::Init(encoding.decode(event)?))
        } else if selector == selector!("ConnOpenAckEvent") {
            Ok(ConnectionHandshakeEvents::Ack(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, InitConnectionEvent>
    for DecodeConnectionHandshakeEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![ClientId, ConnectionId, ClientId]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<InitConnectionEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![client_id_on_a, connection_id_on_a, client_id_on_b] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(InitConnectionEvent {
            client_id_on_a,
            connection_id_on_a,
            client_id_on_b,
        })
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ConnOpenAckEvent>
    for DecodeConnectionHandshakeEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![ClientId, ConnectionId, ClientId, ConnectionId]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ConnOpenAckEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![
            client_id_on_a,
            connection_id_on_a,
            client_id_on_b,
            connection_id_on_b
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        if !event.data.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(ConnOpenAckEvent {
            client_id_on_a,
            connection_id_on_a,
            client_id_on_b,
            connection_id_on_b,
        })
    }
}
