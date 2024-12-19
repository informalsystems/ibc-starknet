use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_components::traits::commitment_prefix::HasCommitmentPrefixType;
use hermes_chain_components::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasClientIdType, HasConnectionIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::connection::{
    ConnectionOpenInitPayload, ConnectionOpenTryPayload,
};
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::connection::types::version::Version as CosmosConnectionVersion;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;
use prost_types::Any;

use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::cosmos::client_state::CometClientState;
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
    Chain: HasMessageType<Message = CosmosMessage>
        + HasHeightType<Height = CosmosHeight>
        + HasClientIdType<Counterparty, ClientId = CosmosClientId>
        + CanRaiseError<IdentifierError>
        + CanRaiseError<ClientError>
        + HasClientStateType<Counterparty, ClientState = CometClientState>,
    Counterparty: HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        > + HasClientIdType<Chain, ClientId = StarknetClientId>
        + HasConnectionIdType<Chain, ConnectionId = StarknetConnectionId>
        + HasHeightType<Height = u64>
        + HasConnectionEndType<Chain>
        + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &CosmosClientId,
        counterparty_client_id: &StarknetClientId,
        counterparty_connection_id: &StarknetConnectionId,
        counterparty_payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let counterparty_client_id_as_cosmos = CosmosClientId::new(
            String::from_utf8_lossy(&counterparty_client_id.client_type.to_bytes_be())
                .trim_start_matches('\0'),
            counterparty_client_id.sequence,
        )
        .map_err(Chain::raise_error)?;

        let counterparty_connection_id_as_cosmos = counterparty_connection_id
            .connection_id
            .as_str()
            .parse()
            .map_err(Chain::raise_error)?;

        // TODO(rano): dummy client state.
        // we need to replace CometClientState with real tendermint ClientState
        let client_state_any = Any {
            type_url: "".to_string(),
            value: vec![],
        };

        // TODO(rano): dummy connection version
        let counterparty_versions = CosmosConnectionVersion::compatibles();

        // TODO(rano): delay period
        let delay_period = Duration::from_secs(0);

        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id_as_cosmos,
            counterparty_connection_id: counterparty_connection_id_as_cosmos,
            counterparty_commitment_prefix: counterparty_payload.commitment_prefix,
            counterparty_versions,
            client_state: client_state_any,
            delay_period,
            update_height,
            proof_init: counterparty_payload.proof_init.proof_bytes,
            proof_client: counterparty_payload.proof_client.proof_bytes,
            proof_consensus: counterparty_payload.proof_consensus.proof_bytes,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
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
