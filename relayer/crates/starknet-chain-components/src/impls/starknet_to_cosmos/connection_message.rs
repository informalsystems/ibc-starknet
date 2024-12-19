use crate::types::client_id::ClientId as StarknetClientId;
use cgp::prelude::*;
use hermes_chain_components::traits::commitment_prefix::HasCommitmentPrefixType;
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
use hermes_chain_components::types::payloads::connection::ConnectionOpenInitPayload;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

pub struct BuildStarknetToCosmosConnectionHandshake;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasClientIdType<Counterparty, ClientId = CosmosClientId>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseError<IdentifierError>,
    Counterparty: HasClientIdType<Chain, ClientId = StarknetClientId>
        + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Counterparty>,
        >,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        client_id: &CosmosClientId,
        counterparty_client_id: &StarknetClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let counterparty_client_id_as_cosmos = CosmosClientId::new(
            String::from_utf8_lossy(&counterparty_client_id.client_type.to_bytes_be())
                .trim_start_matches('\0'),
            counterparty_client_id.sequence,
        )
        .map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenInitMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id_as_cosmos,
            counterparty_commitment_prefix: counterparty_payload.commitment_prefix,
            version: init_connection_options.connection_version.clone().into(),
            delay_period: init_connection_options.delay_period,
        };

        Ok(message.to_cosmos_message())
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
