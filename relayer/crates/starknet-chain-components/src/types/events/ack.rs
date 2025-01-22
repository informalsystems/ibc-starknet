use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::types::channel_id::ChannelId;
use crate::types::event::StarknetEvent;
use crate::types::events::packet::WriteAcknowledgementEvent;
use crate::types::messages::ibc::channel::PortId;
use crate::types::messages::ibc::packet::{Acknowledgement, Sequence};

pub struct DecodeWriteAckEvent;

impl<Encoding, Strategy, CairoEncoding> Decoder<Encoding, Strategy, WriteAcknowledgementEvent>
    for DecodeWriteAckEvent
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Sequence, PortId, ChannelId, PortId, ChannelId]>
        + CanDecode<ViaCairo, Product![Vec<Felt>, Acknowledgement]>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<WriteAcknowledgementEvent, Encoding::Error> {
        let cairo_encoding = encoding.encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
        ] = cairo_encoding
            .decode(&event.keys)
            .map_err(Encoding::raise_error)?;

        let product![packet_data, acknowledgement,] = cairo_encoding
            .decode(&event.data)
            .map_err(Encoding::raise_error)?;

        Ok(WriteAcknowledgementEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            packet_data,
            acknowledgement,
        })
    }
}
