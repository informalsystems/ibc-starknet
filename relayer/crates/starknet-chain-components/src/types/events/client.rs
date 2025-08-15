use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::encoding_components::traits::{
    CanDecode, Decoder, DecoderComponent, HasEncodedType, HasEncoding,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::StarknetUpdateClientEvent;
use crate::types::{Height, StarknetEvent, UnknownEvent};

#[derive(Debug)]
pub enum ClientRelayEvents {
    UpdateClient(StarknetUpdateClientEvent),
}

pub struct DecodeClientRelayEvents;

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy> Decoder<Encoding, Strategy, ClientRelayEvents> for DecodeClientRelayEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, StarknetUpdateClientEvent>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<ClientRelayEvents, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("UpdateClientEvent") {
            Ok(ClientRelayEvents::UpdateClient(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, StarknetUpdateClientEvent> for DecodeClientRelayEvents
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
