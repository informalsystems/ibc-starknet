use core::convert::Infallible;
use core::num::ParseIntError;

use cgp::prelude::*;
use hermes_chain_components::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenTryMessageBuilder,
};
use hermes_chain_components::traits::types::channel::{
    HasChannelEndType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::{ChannelId as IbcChannelId, PortId as IbcPortId};

use crate::types::channel_id::{
    ChannelCounterparty, ChannelEnd, ChannelEnd as StarknetChannelEnd,
    ChannelId as StarknetChannelId, ChannelState,
};
use crate::types::commitment_proof::StarknetCommitmentProof;

pub struct BuildStarknetToCosmosChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + CanRaiseAsyncError<Infallible>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ParseIntError>
        + CanRaiseAsyncError<IdentifierError>,
    Counterparty: HasChannelIdType<Chain, ChannelId = StarknetChannelId>
        + HasPortIdType<Chain, PortId = IbcPortId>
        + HasHeightType<Height = u64>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasChannelEndType<Chain, ChannelEnd = StarknetChannelEnd>
        + HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Chain>,
        >,
{
    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &IbcPortId,
        counterparty_port_id: &IbcPortId,
        counterparty_channel_id: &StarknetChannelId,
        counterparty_payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let starknet_channel_end = counterparty_payload.channel_end;

        let remote = ChannelCounterparty {
            port_id: counterparty_port_id.clone(),
            channel_id: Some(counterparty_channel_id.clone()),
        };

        // building expected channel_end at counterparty
        let channel_end = ChannelEnd {
            state: ChannelState::TryOpen,
            ordering: starknet_channel_end.ordering,
            remote,
            connection_hops: starknet_channel_end.connection_hops,
            version: starknet_channel_end.version.clone(),
        };

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel: channel_end.into(),
            counterparty_version: starknet_channel_end.version.to_string(),
            update_height,
            proof_init: counterparty_payload.proof_init.proof_bytes,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasChannelIdType<Counterparty, ChannelId = IbcChannelId>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasChannelIdType<Chain, ChannelId = StarknetChannelId>
        + HasPortIdType<Chain, PortId = IbcPortId>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>
        + HasChannelEndType<Chain, ChannelEnd = StarknetChannelEnd>
        + HasChannelOpenAckPayloadType<
            Chain,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Counterparty, Chain>,
        >,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
        counterparty_channel_id: &StarknetChannelId,
        counterparty_payload: ChannelOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let proof_try = counterparty_payload.proof_try.proof_bytes;

        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: counterparty_payload.channel_end.version.to_string(),
            update_height,
            proof_try,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasChannelIdType<Counterparty, ChannelId = IbcChannelId>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasHeightType<Height = u64>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Counterparty>,
        >,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
        counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            update_height,
            proof_ack: counterparty_payload.proof_ack.proof_bytes,
        };

        Ok(message.to_cosmos_message())
    }
}
