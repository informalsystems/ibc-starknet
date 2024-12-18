use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenTryMessageBuilder,
};
use hermes_chain_components::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
};
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;

pub struct BuildStarknetToCosmosChannelHandshakeMessage;

impl<Chain, Counterparty> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosChannelHandshakeMessage
where
    Chain: HasMessageType + HasPortIdType<Counterparty> + HasErrorType,
    Counterparty:
        HasChannelIdType<Chain> + HasPortIdType<Chain> + HasChannelOpenTryPayloadType<Chain>,
{
    async fn build_channel_open_try_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _counterparty_port_id: &Counterparty::PortId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: Counterparty::ChannelOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
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
