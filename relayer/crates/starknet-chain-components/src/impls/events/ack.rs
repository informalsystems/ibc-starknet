use core::marker::PhantomData;

use cgp::prelude::CanRaiseAsyncError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::EventExtractor;
use hermes_chain_components::traits::packet::from_write_ack::PacketFromWriteAckEventBuilder;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::{
    HasWriteAckEvent, ProvideWriteAckEvent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::acknowledgement::{AcknowledgementStatus, StatusValue};
use ibc::core::channel::types::error::ChannelError;
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::host::types::error::IdentifierError;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, WriteAcknowledgementEvent};

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAcknowledgementEvent;
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
            seq_on_a: event.sequence_on_a.sequence.into(),
            port_id_on_a: event
                .port_id_on_a
                .port_id
                .parse()
                .map_err(Chain::raise_error)?,
            chan_id_on_a: event
                .channel_id_on_a
                .channel_id
                .parse()
                .map_err(Chain::raise_error)?,
            port_id_on_b: event
                .port_id_on_b
                .port_id
                .parse()
                .map_err(Chain::raise_error)?,
            chan_id_on_b: event
                .channel_id_on_b
                .channel_id
                .parse()
                .map_err(Chain::raise_error)?,

            // FIXME: make the Cairo contract include these fields in the event
            data: Vec::new(),
            timeout_height_on_b: TimeoutHeight::Never,
            timeout_timestamp_on_b: TimeoutTimestamp::Never,
        };

        Ok(packet)
    }

    async fn build_ack_from_write_ack_event(
        _chain: &Chain,
        _ack: &WriteAcknowledgementEvent,
    ) -> Result<Vec<u8>, Chain::Error> {
        // FIXME: Fix the Cairo contract to return Vec<u8> acknowledgement inside event

        let status =
            AcknowledgementStatus::Success(StatusValue::new("dummy").map_err(Chain::raise_error)?);

        let ack = serde_json::to_vec(&status).map_err(Chain::raise_error)?;

        Ok(ack)
    }
}
