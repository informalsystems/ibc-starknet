use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_chain_components::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_chain_components::traits::types::ibc::{HasClientIdType, HasConnectionIdType};
use hermes_chain_components::traits::types::message::HasMessageType;

pub struct BuildStarknetToCosmosConnectionHandshake;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasClientIdType<Counterparty>
        + HasMessageType
        + HasErrorType,
    Counterparty: HasConnectionOpenInitPayloadType<Chain> + HasClientIdType<Chain>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        _counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType + HasClientIdType<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenTryPayloadType<Chain>
        + HasClientIdType<Chain>
        + HasConnectionIdType<Chain>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType + HasConnectionIdType<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenAckPayloadType<Chain> + HasConnectionIdType<Chain>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType + HasConnectionIdType<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenConfirmPayloadType<Chain>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
