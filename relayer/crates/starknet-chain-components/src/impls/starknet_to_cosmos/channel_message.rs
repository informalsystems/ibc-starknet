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
use ibc::core::channel::types::channel::{
    ChannelEnd as IbcChannelEnd, Counterparty as IbcChannelCounterparty, Order as IbcChannelOrder,
    State as IbcChannelState,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::{ChannelId as IbcChannelId, PortId as IbcPortId};

use crate::types::channel_id::{ChannelEnd as StarknetChannelEnd, ChannelId as StarknetChannelId};
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
        + CanRaiseError<&'static str>
        + CanRaiseError<ParseIntError>
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
        counterparty_port_id: &IbcPortId,
        counterparty_channel_id: &StarknetChannelId,
        counterparty_payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let starknet_channel_end = counterparty_payload.channel_end;

        let ordering = match starknet_channel_end.ordering {
            StarknetChannelOrdering::Ordered => IbcChannelOrder::Ordered,
            StarknetChannelOrdering::Unordered => IbcChannelOrder::Unordered,
        };

        if !starknet_channel_end.remote.channel_id.channel_id.is_empty() {
            return Err(Chain::raise_error(
                "ChannelEnd has non-empty channel_id after chan_open_init",
            ));
        }

        let cosmos_channel_seq = counterparty_channel_id
            .channel_id
            .strip_prefix("channel-")
            .ok_or_else(|| Chain::raise_error("ChannelId does not have the expected prefix"))?
            .parse::<u64>()
            .map_err(Chain::raise_error)?;

        let remote = IbcChannelCounterparty {
            port_id: counterparty_port_id.clone(),
            channel_id: Some(IbcChannelId::new(cosmos_channel_seq)),
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
            state: IbcChannelState::TryOpen,
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
    Chain: HasMessageType<Message = CosmosMessage>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasChannelIdType<Counterparty, ChannelId = IbcChannelId>
        + CanRaiseError<ClientError>,
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
            counterparty_channel_id: counterparty_channel_id.channel_id.clone(),
            counterparty_version: counterparty_payload.channel_end.version.version,
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
        + CanRaiseError<ClientError>,
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
