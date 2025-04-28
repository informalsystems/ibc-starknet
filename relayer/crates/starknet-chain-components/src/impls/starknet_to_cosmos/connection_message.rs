use core::time::Duration;

use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CanQueryChainHeight, ConnectionOpenAckMessageBuilder, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilder, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilder, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilder, ConnectionOpenTryMessageBuilderComponent, HasClientIdType,
    HasClientStateType, HasCommitmentPrefixType, HasCommitmentProofType, HasConnectionEndType,
    HasConnectionIdType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType, HasConsensusStateType,
    HasHeightType, HasInitConnectionOptionsType, HasMessageType,
};
use hermes_core::chain_components::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::{
    CosmosConnectionOpenAckMessage, CosmosConnectionOpenConfirmMessage,
    CosmosConnectionOpenInitMessage, CosmosConnectionOpenTryMessage, CosmosInitConnectionOptions,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::connection::types::version::Version as CosmosConnectionVersion;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{
    ClientId as CosmosClientId, ConnectionId as CosmosConnectionId,
};
use ibc::primitives::proto::Any as IbcProtoAny;
use prost_types::Any;

use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::consensus_state::WasmStarknetConsensusState;
use crate::types::cosmos::client_state::CometClientState;
pub struct BuildStarknetToCosmosConnectionHandshake;

#[cgp_provider(ConnectionOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasClientIdType<Counterparty, ClientId = CosmosClientId>
        + HasMessageType<Message = CosmosMessage>
        + HasAsyncErrorType,
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

#[cgp_provider(ConnectionOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasHeightType<Height = CosmosHeight>
        + HasClientIdType<Counterparty, ClientId = CosmosClientId>
        + CanRaiseAsyncError<ClientError>
        + HasClientStateType<Counterparty, ClientState = CometClientState>,
    Counterparty: HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        > + HasClientIdType<Chain, ClientId = StarknetClientId>
        + HasConnectionIdType<Chain, ConnectionId = StarknetConnectionId>
        + HasHeightType<Height = u64>
        + HasConsensusStateType<Chain, ConsensusState = WasmStarknetConsensusState>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
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

        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

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
            // client and consensus proofs are passed but ignored
            // cosmos/ibc-go#7129
            proof_client: counterparty_payload.proof_client.proof_bytes,
            proof_consensus: counterparty_payload.proof_consensus.proof_bytes,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

#[cgp_provider(ConnectionOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasConnectionIdType<Counterparty, ConnectionId = CosmosConnectionId>
        + HasClientStateType<Counterparty, ClientState = CometClientState>
        + HasHeightType<Height = CosmosHeight>
        + CanRaiseAsyncError<ClientError>,
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
            // client and consensus proofs are passed but ignored
            // cosmos/ibc-go#7129
            proof_client: counterparty_payload.proof_client.proof_bytes,
            proof_consensus: counterparty_payload.proof_consensus.proof_bytes,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

#[cgp_provider(ConnectionOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + CanQueryChainHeight<Height = CosmosHeight>
        + HasConnectionIdType<Counterparty, ConnectionId = CosmosConnectionId>
        + CanRaiseAsyncError<ClientError>,
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
