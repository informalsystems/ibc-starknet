use cgp::prelude::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::HList;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::types::client_id::ClientId;
use crate::types::cosmos::height::Height;
use crate::types::event::{StarknetEvent, UnknownEvent};

pub struct CreateClientEvent {
    pub client_id: ClientId,
    pub height: Height,
}

pub struct DecodeCreateClientEvent;

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, CreateClientEvent>
    for DecodeCreateClientEvent
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>
        + for<'a> CanRaiseError<UnknownEvent<'a>>
        ,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![ClientId, Height]>,
{
    fn decode(
        encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<CreateClientEvent, EventEncoding::Error> {
        let cairo_encoding = encoding.encoding();

        if event.selector != Some(selector!("result")) {
            return Err(EventEncoding::raise_error(UnknownEvent { event }))
        }

        let HList![client_id, height] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(CreateClientEvent { client_id, height })
    }
}
