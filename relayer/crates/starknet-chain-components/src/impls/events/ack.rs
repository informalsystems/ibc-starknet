use core::marker::PhantomData;
use std::str::FromStr;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::extract_data::{EventExtractor, EventExtractorComponent};
use hermes_chain_components::traits::packet::from_write_ack::{
    PacketFromWriteAckEventBuilder, PacketFromWriteAckEventBuilderComponent,
};
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::{
    HasWriteAckEvent, ProvideWriteAckEvent, WriteAckEventComponent,
};
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::apps::transfer::types::{Amount, BaseDenom, Memo, PrefixedDenom, TracePath};
use ibc::core::channel::types::packet::Packet as IbcPacket;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::core::host::types::identifiers::{ChannelId, PortId};
use ibc::primitives::Signer;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::types::events::packet::{PacketRelayEvents, WriteAcknowledgementEvent};
use crate::types::messages::ibc::ibc_transfer::TransferPacketData as CairoTransferPacketData;
use crate::types::messages::ibc::packet::Packet;

#[cgp_provider(WriteAckEventComponent)]
impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAcknowledgementEvent;
}

#[cgp_provider(EventExtractorComponent)]
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

#[cgp_provider(PacketFromWriteAckEventBuilderComponent)]
impl<Chain, Counterparty, Encoding> PacketFromWriteAckEventBuilder<Chain, Counterparty>
    for UseStarknetEvents
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgementEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<serde_json::Error>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = IbcPacket>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, CairoTransferPacketData>,
{
    async fn build_packet_from_write_ack_event(
        chain: &Chain,
        event: &WriteAcknowledgementEvent,
    ) -> Result<IbcPacket, Chain::Error> {
        let ibc_packet = from_cairo_to_cosmos_packet(chain, &event.packet, chain.encoding())?;
        Ok(ibc_packet)
    }

    async fn build_ack_from_write_ack_event(
        _chain: &Chain,
        ack: &WriteAcknowledgementEvent,
    ) -> Result<Vec<u8>, Chain::Error> {
        // FIXME: Fix the Cairo contract to return ByteArray acknowledgement inside event.
        // The Cairo encoding for ByteArray is different from Array<u8>

        let ack_bytes = ack.acknowledgement.ack.clone();

        Ok(ack_bytes)
    }
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

fn from_cairo_to_cosmos_packet<Chain, Encoding>(
    _chain: &Chain,
    packet: &Packet,
    encoding: &Encoding,
) -> Result<IbcPacket, Chain::Error>
where
    Chain: CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<serde_json::Error>,
    Encoding: CanDecode<ViaCairo, CairoTransferPacketData> + HasEncodedType<Encoded = Vec<Felt>>,
{
    let seq_on_a = packet.sequence.into();
    let port_id_on_a = PortId::new(packet.src_port_id.clone()).map_err(Chain::raise_error)?;
    let chan_id_on_a = ChannelId::from_str(&packet.src_channel_id).map_err(Chain::raise_error)?;
    let port_id_on_b = PortId::new(packet.dst_port_id.clone()).map_err(Chain::raise_error)?;
    let chan_id_on_b = ChannelId::from_str(&packet.dst_channel_id).map_err(Chain::raise_error)?;

    let timeout_height = RawHeight {
        revision_number: packet.timeout_height.revision_number,
        revision_height: packet.timeout_height.revision_height,
    };

    let timeout_timestamp_on_b = (packet.timeout_timestamp).into();

    let cairo_transfer_packet_data = encoding.decode(&packet.data).map_err(Chain::raise_error)?;

    let memo = Memo::from(cairo_transfer_packet_data.memo.to_string());

    let sender = Signer::from(cairo_transfer_packet_data.sender.to_string());

    let receiver = Signer::from(cairo_transfer_packet_data.receiver.to_string());

    let raw_denom = cairo_transfer_packet_data.denom.to_string();

    let mut parts: Vec<&str> = raw_denom.split('/').collect();
    if !parts.is_empty() {
        parts.pop();
    }
    let trace_path_str = parts.join("/");

    let trace_path = TracePath::from_str(&trace_path_str).map_err(Chain::raise_error)?;

    let denom = PrefixedDenom {
        trace_path,
        base_denom: BaseDenom::from_str(&cairo_transfer_packet_data.denom.base.to_string())
            .map_err(Chain::raise_error)?,
    };

    let amount = Amount::from_str(&cairo_transfer_packet_data.amount.to_string())
        .map_err(Chain::raise_error)?;

    let raw_data = DummyTransferData {
        denom: denom.to_string(),
        amount: amount.to_string(),
        sender: sender.to_string(),
        receiver: receiver.to_string(),
        memo: memo.to_string(),
    };

    let data = serde_json::to_vec(&raw_data).map_err(Chain::raise_error)?;

    Ok(IbcPacket {
        seq_on_a,
        port_id_on_a,
        chan_id_on_a,
        port_id_on_b,
        chan_id_on_b,
        data,
        timeout_height_on_b: timeout_height.try_into().map_err(Chain::raise_error)?,
        timeout_timestamp_on_b,
    })
}
