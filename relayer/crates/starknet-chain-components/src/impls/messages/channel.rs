use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenAckMessageBuilderComponent,
    ChannelOpenConfirmMessageBuilder, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilder, ChannelOpenInitMessageBuilderComponent,
    ChannelOpenTryMessageBuilder, ChannelOpenTryMessageBuilderComponent,
};
use hermes_chain_components::traits::types::channel::{
    HasChannelEndType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType, HasInitChannelOptionsType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasConnectionIdType, HasPortIdType,
};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::channel::{ChannelEnd, Order as IbcOrder};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId as IbcPortId};
use starknet::core::types::{Call, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::channel_id::ChannelId as StarknetChannelId;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::channel::{
    ChannelOrdering, MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry,
};
use crate::types::messages::ibc::packet::StateProof;
pub struct BuildStarknetChannelHandshakeMessages;

#[cgp_provider(ChannelOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasPortIdType<Chain, PortId = IbcPortId>,
    Encoding: CanEncode<ViaCairo, MsgChanOpenInit> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_init_message(
        chain: &Chain,
        port_id: &IbcPortId,
        counterparty_port_id: &IbcPortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        if init_channel_options.connection_hops.len() != 1 {
            return Err(Chain::raise_error(
                "Starknet only supports a single connection hop",
            ));
        }

        let conn_id_on_a = init_channel_options.connection_hops[0].clone();

        let ordering = match init_channel_options.ordering {
            IbcOrder::None => {
                return Err(Chain::raise_error("Starknet does not support no ordering"))
            }
            IbcOrder::Ordered => ChannelOrdering::Ordered,
            IbcOrder::Unordered => ChannelOrdering::Unordered,
        };

        let chan_open_init_msg = MsgChanOpenInit {
            port_id_on_a: port_id.clone(),
            conn_id_on_a,
            port_id_on_b: counterparty_port_id.clone(),
            version_proposal: init_channel_options.channel_version.clone(),
            ordering,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_init_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("chan_open_init"),
            calldata,
        };

        let message = StarknetMessage::new(call);

        Ok(message)
    }
}

#[cgp_provider(ChannelOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasChannelIdType<Chain, ChannelId = ChannelId>
        + HasHeightType<Height = Height>
        + HasPortIdType<Chain, PortId = IbcPortId>
        + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Chain>,
        >,
    Encoding: CanEncode<ViaCairo, MsgChanOpenTry> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_try_message(
        chain: &Chain,
        port_id: &IbcPortId,
        counterparty_port_id: &IbcPortId,
        counterparty_channel_id: &ChannelId,
        payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        if payload.channel_end.connection_hops.len() != 1 {
            return Err(Chain::raise_error(
                "Starknet only supports a single connection hop",
            ));
        }

        let conn_id_on_b = payload.counterparty_connection_id;

        let proof_chan_end_on_a = StateProof {
            proof: payload.proof_init.proof_bytes.clone(),
        };

        let proof_height_on_a = CairoHeight {
            revision_number: payload.update_height.revision_number(),
            revision_height: payload.update_height.revision_height(),
        };

        let ordering = match payload.channel_end.ordering {
            IbcOrder::None => {
                return Err(Chain::raise_error("Starknet does not support no ordering"))
            }
            IbcOrder::Ordered => ChannelOrdering::Ordered,
            IbcOrder::Unordered => ChannelOrdering::Unordered,
        };

        let chan_open_try_msg = MsgChanOpenTry {
            port_id_on_b: port_id.clone(),
            conn_id_on_b,
            port_id_on_a: counterparty_port_id.clone(),
            chan_id_on_a: counterparty_channel_id.clone(),
            version_on_a: payload.channel_end.version.clone(),
            proof_chan_end_on_a,
            proof_height_on_a,
            ordering,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_try_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("chan_open_try"),
            calldata,
        };

        let message = StarknetMessage::new(call).with_counterparty_height(payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(ChannelOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasChannelIdType<Counterparty, ChannelId = StarknetChannelId>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasChannelOpenAckPayloadType<
            Chain,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Counterparty, Chain>,
        > + HasChannelIdType<Chain, ChannelId = ChannelId>
        + HasHeightType<Height = Height>
        + HasPortIdType<Chain, PortId = IbcPortId>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>,
    Encoding: CanEncode<ViaCairo, MsgChanOpenAck> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_ack_message(
        chain: &Chain,
        port_id: &IbcPortId,
        channel_id: &StarknetChannelId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: ChannelOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let proof_chan_end_on_b = StateProof {
            proof: counterparty_payload.proof_try.proof_bytes.clone(),
        };

        let proof_height_on_b = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let chan_open_ack_msg = MsgChanOpenAck {
            port_id_on_a: port_id.clone(),
            chan_id_on_a: channel_id.clone(),
            chan_id_on_b: counterparty_channel_id.clone(),
            version_on_b: counterparty_payload.channel_end.version.clone(),
            proof_chan_end_on_b,
            proof_height_on_b,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_ack_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("chan_open_ack"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(ChannelOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasChannelIdType<Counterparty, ChannelId = StarknetChannelId>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Counterparty>,
        > + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>,
    Encoding: CanEncode<ViaCairo, MsgChanOpenConfirm> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_confirm_message(
        chain: &Chain,
        port_id: &IbcPortId,
        channel_id: &StarknetChannelId,
        counterparty_payload: ChannelOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let proof_chan_end_on_a = StateProof {
            proof: counterparty_payload.proof_ack.proof_bytes.clone(),
        };

        let proof_height_on_a = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let chan_open_confirm_msg = MsgChanOpenConfirm {
            port_id_on_b: port_id.clone(),
            chan_id_on_b: channel_id.clone(),
            proof_chan_end_on_a,
            proof_height_on_a,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_confirm_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("chan_open_confirm"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}
