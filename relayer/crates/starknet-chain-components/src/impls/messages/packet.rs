use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_chain_components::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_chain_components::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::HasAckPacketPayloadType;
use hermes_chain_components::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_chain_components::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::packet::ReceivePacketPayload;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::packet::Packet as IbcPacket;
use ibc::core::client::types::Height;
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::packet::{MsgRecvPacket, Packet as CairoPacket, StateProof};

pub struct BuildStarknetPacketMessages;

impl<Chain, Counterparty, Encoding> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = IbcPacket>
        + HasHeightType<Height = Height>
        + HasCommitmentProofType
        + HasReceivePacketPayloadType<
            Chain,
            ReceivePacketPayload = ReceivePacketPayload<Counterparty>,
        >,
    Encoding: CanEncode<ViaCairo, MsgRecvPacket> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_receive_packet_message(
        chain: &Chain,
        packet: &Counterparty::OutgoingPacket,
        counterparty_payload: ReceivePacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        // FIXME: commitment proof should be in the ByteArray format, not Vec<Felt>
        let proof_commitment_on_a = StateProof {
            proof: vec![Felt::ONE],
        };

        let proof_height_on_a = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let receive_packet_msg = MsgRecvPacket {
            packet: CairoPacket::from(packet.clone()),
            proof_commitment_on_a,
            proof_height_on_a,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&receive_packet_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: ibc_core_address,
            selector: selector!("recv_packet"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

impl<Chain, Counterparty> AckPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType,
    Counterparty: HasAckPacketPayloadType<Chain>,
{
    async fn build_ack_packet_message(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
        _payload: Counterparty::AckPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
    for BuildStarknetPacketMessages
where
    Chain: HasMessageType + HasOutgoingPacketType<Counterparty> + HasErrorType,
    Counterparty: HasTimeoutUnorderedPacketPayloadType<Chain>,
{
    async fn build_timeout_unordered_packet_message(
        _chain: &Chain,
        _packet: &Chain::OutgoingPacket,
        _payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
