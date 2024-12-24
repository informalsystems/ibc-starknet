use std::convert::Infallible;

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
use hermes_chain_components::types::payloads::channel::ChannelOpenTryPayload;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
use ibc::core::channel::types::channel::{
    ChannelEnd as IbcChannelEnd, Counterparty as IbcChannelCounterparty, Order as IbcChannelOrder,
    State as IbcChannelState,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::PortId as IbcPortId;

use crate::types::channel_id::{
    ChannelEnd as StarknetChannelEnd, ChannelId as StarknetChannelId,
    ChannelState as StarknetChannelState,
};
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::messages::ibc::channel::ChannelOrdering as StarknetChannelOrdering;

pub struct BuildStarknetToCosmosChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + CanRaiseError<Infallible>
        + CanRaiseError<ClientError>
        + CanRaiseError<IdentifierError>,
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
        _counterparty_port_id: &IbcPortId,
        _counterparty_channel_id: &StarknetChannelId,
        counterparty_payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let starknet_channel_end = counterparty_payload.channel_end;

        let state = match starknet_channel_end.state {
            StarknetChannelState::Uninitialized => IbcChannelState::Uninitialized,
            StarknetChannelState::Init => IbcChannelState::Init,
            StarknetChannelState::TryOpen => IbcChannelState::TryOpen,
            StarknetChannelState::Open => IbcChannelState::Open,
            StarknetChannelState::Closed => IbcChannelState::Closed,
        };

        let ordering = match starknet_channel_end.ordering {
            StarknetChannelOrdering::Ordered => IbcChannelOrder::Ordered,
            StarknetChannelOrdering::Unordered => IbcChannelOrder::Unordered,
        };

        let remote = IbcChannelCounterparty {
            port_id: starknet_channel_end
                .remote
                .port_id
                .port_id
                .parse()
                .map_err(Chain::raise_error)?,
            channel_id: Some(
                starknet_channel_end
                    .remote
                    .channel_id
                    .channel_id
                    .parse()
                    .map_err(Chain::raise_error)?,
            ),
        };

        let connection_id = starknet_channel_end
            .connection_id
            .connection_id
            .parse()
            .map_err(Chain::raise_error)?;

        let version = starknet_channel_end
            .version
            .version
            .parse()
            .map_err(Chain::raise_error)?;

        // TODO(rano): how to get channel end here ?
        let channel_end = IbcChannelEnd {
            state,
            ordering,
            remote,
            connection_hops: vec![connection_id],
            version,
        };

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.to_string(),
            channel: channel_end.into(),
            counterparty_version: starknet_channel_end.version.version,
            update_height,
            proof_init: counterparty_payload.proof_init.proof_bytes,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasErrorType,
    Counterparty:
        HasChannelIdType<Chain> + HasPortIdType<Chain> + HasChannelOpenAckPayloadType<Chain>,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<Chain>,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
