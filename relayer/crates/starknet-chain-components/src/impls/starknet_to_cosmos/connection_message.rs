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
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::connection::types::version::Version as CosmosConnectionVersion;
use ibc::core::host::types::identifiers::{
    ClientId as CosmosClientId, ConnectionId as CosmosConnectionId,
};
use ibc::primitives::proto::Any as IbcProtoAny;
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
        + HasErrorType,
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
        let message = CosmosConnectionOpenInitMessage {
            client_id: client_id.to_string(),
            counterparty_client_id: counterparty_client_id.to_string(),
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
        let ibc_client_state_any = IbcProtoAny::from(counterparty_payload.client_state);

        // TODO(rano): dummy connection version
        let counterparty_versions = CosmosConnectionVersion::compatibles();

        // TODO(rano): delay period
        let delay_period = Duration::from_secs(0);

        // TODO(rano): apparently update height is set to zero
        let update_height =
            CosmosHeight::new(0, core::cmp::max(1, counterparty_payload.update_height))
                .map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.to_string(),
            counterparty_client_id: counterparty_client_id.to_string(),
            counterparty_connection_id: counterparty_connection_id.to_string(),
            counterparty_commitment_prefix: counterparty_payload.commitment_prefix,
            counterparty_versions,
            client_state: Any {
                type_url: ibc_client_state_any.type_url,
                value: ibc_client_state_any.value,
            },
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
    Chain: HasMessageType<Message = CosmosMessage>
        + HasConnectionIdType<Counterparty, ConnectionId = CosmosConnectionId>
        + HasClientStateType<Counterparty, ClientState = CometClientState>
        + HasHeightType<Height = CosmosHeight>
        + CanRaiseError<ClientError>,
    Counterparty: HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Counterparty, Chain>,
        > + HasConnectionIdType<Chain, ConnectionId = StarknetConnectionId>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>
        + HasConnectionEndType<Chain>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &CosmosConnectionId,
        counterparty_connection_id: &StarknetConnectionId,
        counterparty_payload: ConnectionOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let client_state_any = IbcProtoAny::from(counterparty_payload.client_state);

        // TODO(rano): dummy connection version
        let counterparty_versions = CosmosConnectionVersion::compatibles();

        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenAckMessage {
            connection_id: connection_id.to_string(),
            counterparty_connection_id: counterparty_connection_id.to_string(),
            version: counterparty_versions[0].clone().into(),
            client_state: Any {
                type_url: client_state_any.type_url,
                value: client_state_any.value,
            },
            update_height,
            proof_try: counterparty_payload.proof_try.proof_bytes,
            proof_client: counterparty_payload.proof_client.proof_bytes,
            proof_consensus: counterparty_payload.proof_consensus.proof_bytes,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasConnectionIdType<Counterparty, ConnectionId = CosmosConnectionId>
        + CanRaiseError<ClientError>,
    Counterparty: HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        > + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &CosmosConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.to_string(),
            update_height,
            proof_ack: counterparty_payload.proof_ack.proof_bytes,
        };

        Ok(message.to_cosmos_message())
    }
}
