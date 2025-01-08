use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::ProvideWriteAckEvent;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::types::channel_id::ChannelId;
use crate::types::event::StarknetEvent;
use crate::types::events::packet::WriteAcknowledgementEvent;
use crate::types::messages::ibc::channel::PortId;
use crate::types::messages::ibc::packet::{Acknowledgement, Sequence};

pub struct UseStarknetWriteAckEvent;

impl<Chain, Counterparty, Encoding> ProvideWriteAckEvent<Chain, Counterparty>
    for UseStarknetWriteAckEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Sequence, PortId, ChannelId, PortId, ChannelId]>
        + CanDecode<ViaCairo, Product![Vec<Felt>, Acknowledgement]>,
{
    type WriteAckEvent = WriteAcknowledgementEvent;

    fn try_extract_write_ack_event(event: &StarknetEvent) -> Option<Self::WriteAckEvent> {
        // TODO(rano): don't have access to the EventEncoding
        // Ideally, EventEncoding to decode directly from StarknetEvent

        let cairo_encoding = Chain::default_encoding();

        let product![
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
        ] = cairo_encoding.decode(&event.keys).ok()?;

        let product![packet_data, acknowledgement,] = cairo_encoding.decode(&event.data).ok()?;

        Some(WriteAcknowledgementEvent {
            sequence_on_a,
            port_id_on_a,
            channel_id_on_a,
            port_id_on_b,
            channel_id_on_b,
            packet_data,
            acknowledgement,
        })
    }

    fn write_acknowledgement(ack: &WriteAcknowledgementEvent) -> impl AsRef<Vec<u8>> + Send {
        ack.acknowledgement.ack.clone()
    }
}
