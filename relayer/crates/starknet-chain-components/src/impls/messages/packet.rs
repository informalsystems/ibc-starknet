use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::ack_packet::{
    AckPacketMessageBuilder, AckPacketMessageBuilderComponent,
};
use hermes_chain_components::traits::message_builders::receive_packet::{
    ReceivePacketMessageBuilder, ReceivePacketMessageBuilderComponent,
};
use hermes_chain_components::traits::message_builders::timeout_unordered_packet::{
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketMessageBuilderComponent,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::{
    HasAckPacketPayloadType, HasAcknowledgementType,
};
use hermes_chain_components::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_chain_components::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::apps::transfer::types::packet::PacketData as IbcIcs20PacketData;
use ibc::core::channel::types::packet::Packet as IbcPacket;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use starknet::core::types::{Call, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};
use crate::types::messages::ibc::ibc_transfer::{
    Participant, TransferPacketData as CairoTransferPacketData,
};
use crate::types::messages::ibc::packet::{
    Acknowledgement as CairoAck, MsgAckPacket, MsgRecvPacket, MsgTimeoutPacket,
    Packet as CairoPacket, StateProof,
};

pub struct BuildStarknetPacketMessages;

#[cgp_provider(ReceivePacketMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = IbcPacket>
        + HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasReceivePacketPayloadType<
            Chain,
            ReceivePacketPayload = ReceivePacketPayload<Counterparty>,
        >,
    Encoding: CanEncode<ViaCairo, MsgRecvPacket>
        + CanEncode<ViaCairo, CairoTransferPacketData>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_receive_packet_message(
        chain: &Chain,
        packet: &IbcPacket,
        counterparty_payload: ReceivePacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let proof_commitment_on_a = StateProof {
            proof: counterparty_payload.proof_commitment.proof_bytes.clone(),
        };

        let proof_height_on_a = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let receive_packet_msg = MsgRecvPacket {
            packet: from_cosmos_to_cairo_packet(packet, chain.encoding()),
            proof_commitment_on_a,
            proof_height_on_a,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&receive_packet_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("recv_packet"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(AckPacketMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> AckPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = IbcPacket>
        + HasAsyncErrorType,
    Counterparty: HasAckPacketPayloadType<Chain, AckPacketPayload = AckPacketPayload<Counterparty, Chain>>
        + HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasAcknowledgementType<Chain, Acknowledgement = Vec<u8>>,
    Encoding: CanEncode<ViaCairo, MsgAckPacket>
        + CanEncode<ViaCairo, CairoTransferPacketData>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_ack_packet_message(
        chain: &Chain,
        packet: &IbcPacket,
        counterparty_payload: AckPacketPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let proof_ack_on_b = StateProof {
            proof: counterparty_payload.proof_ack.proof_bytes.clone(),
        };

        let proof_height_on_b = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let ack_packet_msg = MsgAckPacket {
            packet: from_cosmos_to_cairo_packet(packet, chain.encoding()),
            acknowledgement: CairoAck {
                ack: counterparty_payload.ack,
            },
            proof_ack_on_b,
            proof_height_on_b,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&ack_packet_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("ack_packet"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(TimeoutUnorderedPacketMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = IbcPacket>
        + HasAsyncErrorType,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasTimeoutUnorderedPacketPayloadType<
            Chain,
            TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Counterparty>,
        >,
    Encoding: CanEncode<ViaCairo, MsgTimeoutPacket>
        + CanEncode<ViaCairo, CairoTransferPacketData>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_timeout_unordered_packet_message(
        chain: &Chain,
        packet: &IbcPacket,
        counterparty_payload: TimeoutUnorderedPacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let proof_unreceived_on_b = StateProof {
            proof: counterparty_payload.proof_unreceived.proof_bytes.clone(),
        };

        let proof_height_on_b = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let timeout_packet_msg = MsgTimeoutPacket {
            packet: from_cosmos_to_cairo_packet(packet, chain.encoding()),
            // Cairo only accepts unordered packets.
            // So, this sequence is ignored.
            next_seq_recv_on_b: 1.into(),
            proof_unreceived_on_b,
            proof_height_on_b,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&timeout_packet_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("timeout_packet"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

fn from_cosmos_to_cairo_packet<Encoding>(packet: &IbcPacket, encoding: &Encoding) -> CairoPacket
where
    Encoding: CanEncode<ViaCairo, CairoTransferPacketData> + HasEncodedType<Encoded = Vec<Felt>>,
{
    let sequence = packet.seq_on_a.value();
    let src_port_id = packet.port_id_on_a.to_string();
    let src_channel_id = packet.chan_id_on_a.to_string();
    let dst_port_id = packet.port_id_on_b.to_string();
    let dst_channel_id = packet.chan_id_on_b.to_string();

    // TODO(rano): the packet data needs to serialized to Vec<felt>.
    // to do that, we assume PacketData struct (i.e. ICS20) and construct it.
    // ideally, Cairo contract should accept the serialized data directly.

    // deserialize to ibc ics20 packet message

    let ibc_ics20_packet_data: IbcIcs20PacketData = serde_json::from_slice(&packet.data).unwrap();

    // convert to cairo packet message

    // TODO(rano): can't iter. need fix at ibc-rs side
    // for now, using json hack
    let trace_path_json =
        serde_json::to_string(&ibc_ics20_packet_data.token.denom.trace_path).unwrap();

    #[derive(serde::Deserialize)]
    struct DummyTracePath {
        pub port_id: String,
        pub channel_id: String,
    }

    let trace_path: Vec<DummyTracePath> = serde_json::from_str(&trace_path_json).unwrap();

    let denom_string = ibc_ics20_packet_data
        .token
        .denom
        .base_denom
        .as_str()
        .to_string();

    let base_denom = denom_string
        .parse()
        .map(Denom::Native)
        .unwrap_or_else(|_| Denom::Hosted(denom_string));

    let denom = PrefixedDenom {
        trace_path: trace_path
            .into_iter()
            .map(
                |DummyTracePath {
                     port_id,
                     channel_id,
                 }| TracePrefix {
                    port_id,
                    channel_id,
                },
            )
            .collect(),
        base: base_denom,
    };

    let amount = {
        let bytes = ibc_ics20_packet_data.token.amount.as_ref().0;
        crypto_bigint::U256::from(bytes).into()
    };

    let sender_string = ibc_ics20_packet_data.sender.as_ref().to_string();
    let receiver_string = ibc_ics20_packet_data.receiver.as_ref().to_string();

    // TODO(rano): the following is a hack
    // do we really need Participant variants?

    let sender = sender_string
        .parse()
        .map(Participant::Native)
        .unwrap_or_else(|_| Participant::External(sender_string));

    let receiver = receiver_string
        .parse()
        .map(Participant::Native)
        .unwrap_or_else(|_| Participant::External(receiver_string));

    match (&sender, &receiver) {
        (Participant::Native(_), Participant::Native(_)) => {
            panic!("Native to Native transfer is not supported")
        }
        (Participant::External(_), Participant::External(_)) => {
            panic!("External to External transfer is not supported")
        }
        _ => {}
    }

    let memo = ibc_ics20_packet_data.memo.as_ref().to_string();

    let cairo_ics20_packet_data = CairoTransferPacketData {
        denom,
        amount,
        sender,
        receiver,
        memo,
    };

    // serialize to vec<felt>

    let data_felt = encoding.encode(&cairo_ics20_packet_data).unwrap();

    let timeout_height = match packet.timeout_height_on_b {
        TimeoutHeight::Never => CairoHeight {
            revision_number: 0,
            revision_height: 0,
        },
        TimeoutHeight::At(height) => CairoHeight {
            revision_number: height.revision_number(),
            revision_height: height.revision_height(),
        },
    };

    let timeout_timestamp = match packet.timeout_timestamp_on_b {
        TimeoutTimestamp::Never => 0,
        TimeoutTimestamp::At(timeout_timestamp) => timeout_timestamp.nanoseconds(),
    };

    CairoPacket {
        sequence,
        src_port_id,
        src_channel_id,
        dst_port_id,
        dst_channel_id,
        data: data_felt,
        timeout_height,
        timeout_timestamp,
    }
}
