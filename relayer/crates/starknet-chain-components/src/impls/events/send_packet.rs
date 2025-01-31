use core::marker::PhantomData;

use cgp::prelude::CanRaiseAsyncError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::EventExtractor;
use hermes_chain_components::traits::packet::from_send_packet::PacketFromSendPacketEventBuilder;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::send_packet::{
    HasSendPacketEvent, ProvideSendPacketEvent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::apps::transfer::types::packet::PacketData;
use ibc::apps::transfer::types::PrefixedCoin;
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, SendPacketEvent};
use crate::types::messages::ibc::ibc_transfer::TransferPacketData;

impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet> + HasEventType,
{
    type SendPacketEvent = SendPacketEvent;
}

impl<Chain, Counterparty, Encoding> PacketFromSendPacketEventBuilder<Chain, Counterparty>
    for UseStarknetEvents
where
    Chain: HasSendPacketEvent<Counterparty, SendPacketEvent = SendPacketEvent>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<IdentifierError>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, TransferPacketData>,
{
    async fn build_packet_from_send_packet_event(
        chain: &Chain,
        event: &SendPacketEvent,
    ) -> Result<Packet, Chain::Error> {
        let timeout_height_on_b = Height::new(
            event.timeout_height_on_b.revision_number,
            event.timeout_height_on_b.revision_height,
        )
        .map(TimeoutHeight::At)
        .unwrap_or_else(|_| TimeoutHeight::Never);

        let timeout_timestamp_on_b = if event.timeout_timestamp_on_b.nanoseconds() > 0 {
            TimeoutTimestamp::At(event.timeout_timestamp_on_b)
        } else {
            TimeoutTimestamp::Never
        };

        /*
            FIXME: the packet data format in Cairo is incompatible with the packet data on Cosmos.
            The ICS20 transfer packet data is encoded using Cairo encoding into a Vec<Felt>.
            A proper fix to this is to fix the Cairo contract encode the data using JSON or
            Protobuf into Vec<u8>.

            As a workaround, the relayer assume that the packet data is always from Cairo ICS20,
            and perform the re-encoding before submitting to Cosmos.
            Note that this will NOT work once we implement membership proof verification,
            because there is no way the Cosmos client can verify that the re-encoding by the
            relayer is correct. This approach also does not support relaying multiple IBC
            applications with different packet data format.
        */

        let starknet_transfer_packet_data = chain
            .encoding()
            .decode(&event.packet_data)
            .map_err(Chain::raise_error)?;

        let cosmos_ibc_transfer_packet_data = PacketData {
            token: PrefixedCoin {
                denom: starknet_transfer_packet_data
                    .denom
                    .to_string()
                    .parse()
                    .map_err(Chain::raise_error)?,
                amount: starknet_transfer_packet_data
                    .amount
                    .to_string()
                    .parse()
                    .map_err(Chain::raise_error)?,
            },
            sender: starknet_transfer_packet_data.sender.to_string().into(),
            receiver: starknet_transfer_packet_data.receiver.to_string().into(),
            memo: starknet_transfer_packet_data.memo.into(),
        };

        let cosmos_packet_data =
            serde_json::to_vec(&cosmos_ibc_transfer_packet_data).map_err(Chain::raise_error)?;

        let packet = Packet {
            seq_on_a: event.sequence_on_a.sequence.into(),
            port_id_on_a: event.port_id_on_a.clone(),
            chan_id_on_a: event.channel_id_on_a.clone(),
            port_id_on_b: event.port_id_on_b.clone(),
            chan_id_on_b: event.channel_id_on_b.clone(),
            data: cosmos_packet_data,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        Ok(packet)
    }
}

impl<Chain, Encoding> EventExtractor<Chain, SendPacketEvent> for UseStarknetEvents
where
    Chain: HasEventType + HasEncoding<AsStarknetEvent, Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Chain::Event> + CanDecode<ViaCairo, Option<PacketRelayEvents>>,
{
    fn try_extract_from_event(
        chain: &Chain,
        _tag: PhantomData<SendPacketEvent>,
        raw_event: &Chain::Event,
    ) -> Option<SendPacketEvent> {
        let event = chain.encoding().decode(raw_event).ok()??;

        match event {
            PacketRelayEvents::Send(event) => Some(event),
            _ => None,
        }
    }
}
