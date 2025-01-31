use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_components::traits::commitment_prefix::{
    HasCommitmentPrefixType, HasIbcCommitmentPrefix,
};
use hermes_chain_components::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_chain_components::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_chain_components::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_chain_components::traits::types::consensus_state::HasConsensusStateType;
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
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_cosmos_chain_components::types::status::ChainStatus;
use hermes_relayer_components::transaction::traits::default_signer::HasDefaultSigner;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::connection::types::version::Version as CosmosConnectionVersion;
use ibc::core::connection::types::{
    ConnectionEnd, Counterparty as ConnectionCounterparty, State as ConnectionState,
};
use ibc::core::host::types::identifiers::{
    ClientId as CosmosClientId, ConnectionId as CosmosConnectionId,
};
use ibc::core::host::types::path::{ConnectionPath, Path};
use ibc::primitives::proto::Any as IbcProtoAny;
use ibc_proto::Protobuf;
use prost_types::Any;

use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::commitment_proof::StarknetCommitmentProof;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::consensus_state::WasmStarknetConsensusState;
use crate::types::cosmos::client_state::CometClientState;
use crate::types::membership_proof_signer::MembershipVerifierContainer;
pub struct BuildStarknetToCosmosConnectionHandshake;

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

impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasHeightType<Height = CosmosHeight>
        + HasClientIdType<Counterparty, ClientId = CosmosClientId>
        + CanRaiseAsyncError<ClientError>
        + CanQueryChainStatus<ChainStatus = ChainStatus>
        + CanQueryConsensusStateWithLatestHeight<Counterparty>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<&'static str>
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
        chain: &Chain,
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

        let proof_init = {
            let data = MembershipVerifierContainer {
                // FIXME(hack) we are passing consensus root as proof
                state_root: counterparty_payload.proof_init.proof_bytes,
                prefix: counterparty_payload.commitment_prefix.clone(),
                path: Path::Connection(ConnectionPath::new(counterparty_connection_id))
                    .to_string()
                    .into(),
                value: Some(counterparty_payload.connection_end.encode_vec()),
            };

            chain
                .get_default_signer()
                .sign(&data.canonical_bytes())
                .map_err(Chain::raise_error)?
        };

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
            proof_init,
            proof_client: b"ignored".into(),
            proof_consensus: b"ignored".into(),
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
            // FIXME: build the open_ack_proof similar to open_confirm_proof
            proof_try: counterparty_payload.proof_try.proof_bytes,
            proof_client: b"ignored".into(),
            proof_consensus: b"ignored".into(),
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetToCosmosConnectionHandshake
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + CanQueryChainHeight<Height = CosmosHeight>
        + CanQueryConnectionEnd<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasConnectionIdType<Counterparty, ConnectionId = CosmosConnectionId>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        > + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasHeightType<Height = u64>,
{
    async fn build_connection_open_confirm_message(
        chain: &Chain,
        connection_id: &CosmosConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height =
            CosmosHeight::new(0, counterparty_payload.update_height).map_err(Chain::raise_error)?;

        let cosmos_latest_height = chain.query_chain_height().await?;

        let connection_end_at_cosmos = chain
            .query_connection_end(connection_id, &cosmos_latest_height)
            .await?;

        let cosmos_prefix = chain.ibc_commitment_prefix();

        let counterparty_at_starknet = ConnectionCounterparty::new(
            connection_end_at_cosmos.client_id().clone(),
            Some(connection_id.clone()),
            cosmos_prefix.clone().into(),
        );

        let connection_end_at_starknet = ConnectionEnd::new(
            ConnectionState::Open,
            connection_end_at_cosmos.counterparty().client_id().clone(),
            counterparty_at_starknet,
            connection_end_at_cosmos.versions().to_vec(),
            connection_end_at_cosmos.delay_period(),
        )
        .map_err(|_| Chain::raise_error("invalid connection end"))?;

        let connection_id_at_starknet = connection_end_at_cosmos
            .counterparty()
            .connection_id()
            .ok_or_else(|| Chain::raise_error("missing connection id"))?;

        let prefix_at_starknet = connection_end_at_cosmos.counterparty().prefix();

        let proof_ack = {
            let data = MembershipVerifierContainer {
                // FIXME(hack) we are passing consensus root as proof
                state_root: counterparty_payload.proof_ack.proof_bytes,
                prefix: prefix_at_starknet.clone().into_vec(),
                path: Path::Connection(ConnectionPath::new(connection_id_at_starknet))
                    .to_string()
                    .into(),
                value: Some(connection_end_at_starknet.encode_vec()),
            };

            chain
                .get_default_signer()
                .sign(&data.canonical_bytes())
                .map_err(Chain::raise_error)?
        };

        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.to_string(),
            update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}
