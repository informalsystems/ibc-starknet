use core::marker::PhantomData;
use std::str::FromStr;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::{EventExtractor, EventExtractorComponent};
use hermes_chain_components::traits::packet::from_send_packet::{
    PacketFromSendPacketEventBuilder, PacketFromSendPacketEventBuilderComponent,
};
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::send_packet::{
    HasSendPacketEvent, ProvideSendPacketEvent, SendPacketEventComponent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::apps::transfer::types::{Amount, BaseDenom, Memo, PrefixedDenom, TracePath};
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::primitives::Signer;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, SendPacketEvent};
use crate::types::messages::ibc::ibc_transfer::TransferPacketData;

#[cgp_provider(SendPacketEventComponent)]
impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet> + HasEventType,
{
    type SendPacketEvent = SendPacketEvent;
}

// FIXME: Fix conversion from Cairo to Cosmos packet
#[derive(Serialize, Deserialize)]
pub struct DummyTransferData {
    pub amount: String,
    pub denom: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub memo: String,
    pub receiver: String,
    pub sender: String,
}

#[cgp_provider(PacketFromSendPacketEventBuilderComponent)]
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

            As a workaround, is using Dummy Transfer Data which has the correct order.
        */

        let starknet_transfer_packet_data = chain
            .encoding()
            .decode(&event.packet_data)
            .map_err(Chain::raise_error)?;

        let memo = Memo::from(starknet_transfer_packet_data.memo.to_string());

        let sender = Signer::from(starknet_transfer_packet_data.sender.to_string());

        let receiver = Signer::from(starknet_transfer_packet_data.receiver.to_string());

        let raw_denom = starknet_transfer_packet_data.denom.to_string();

        let mut parts: Vec<&str> = raw_denom.split('/').collect();
        if !parts.is_empty() {
            parts.pop();
        }
        let trace_path_str = parts.join("/");

        let trace_path = TracePath::from_str(&trace_path_str).map_err(Chain::raise_error)?;

        let denom = PrefixedDenom {
            trace_path,
            base_denom: BaseDenom::from_str(&starknet_transfer_packet_data.denom.base.to_string())
                .map_err(Chain::raise_error)?,
        };

        let amount = Amount::from_str(&starknet_transfer_packet_data.amount.to_string())
            .map_err(Chain::raise_error)?;

        let raw_data = DummyTransferData {
            denom: denom.to_string(),
            amount: amount.to_string(),
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            memo: memo.to_string(),
        };

        let cosmos_packet_data = serde_json::to_vec(&raw_data).map_err(Chain::raise_error)?;

        let packet = Packet {
            seq_on_a: event.sequence_on_a,
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

#[cgp_provider(EventExtractorComponent)]
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
