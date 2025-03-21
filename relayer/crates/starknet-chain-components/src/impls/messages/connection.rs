use core::marker::PhantomData;
use core::num::ParseIntError;
use core::str::Utf8Error;
use core::time::Duration;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::commitment_prefix::HasCommitmentPrefixType;
use hermes_chain_components::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilder, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilder, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilder, ConnectionOpenTryMessageBuilderComponent,
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
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{
    ClientId as CosmosClientId, ConnectionId as CosmosConnectionId,
};
use starknet::core::types::{Call, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::connection::{
    ConnectionVersion, MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
};
use crate::types::messages::ibc::packet::StateProof;

pub struct BuildStarknetConnectionHandshakeMessages;

#[cgp_provider(ConnectionOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasClientIdType<Counterparty, ClientId = StarknetClientId>
        + HasAddressType<Address = StarknetAddress>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<ParseIntError>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<Utf8Error>,
    Counterparty: HasClientIdType<Chain, ClientId = CosmosClientId>
        + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Counterparty>,
        >,
    Encoding: CanEncode<ViaCairo, MsgConnOpenInit> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_connection_open_init_message(
        chain: &Chain,
        client_id: &StarknetClientId,
        counterparty_client_id: &CosmosClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let conn_open_init_msg = MsgConnOpenInit {
            client_id_on_a: client_id.clone(),
            client_id_on_b: counterparty_client_id.clone(),
            prefix_on_b: counterparty_payload.commitment_prefix.into(),
            version: init_connection_options.connection_version.clone(),
            delay_period: init_connection_options.delay_period,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_init_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("conn_open_init"),
            calldata,
        };

        let message = StarknetMessage::new(call);

        Ok(message)
    }
}

#[cgp_provider(ConnectionOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasHeightType
        + HasMessageType<Message = StarknetMessage>
        + HasClientIdType<Counterparty, ClientId = StarknetClientId>
        + HasClientStateType<Counterparty>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<ParseIntError>
        + CanRaiseAsyncError<Utf8Error>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasClientIdType<Chain, ClientId = CosmosClientId>
        + HasConnectionIdType<Chain, ConnectionId = CosmosConnectionId>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        >,
    Encoding: CanEncode<ViaCairo, MsgConnOpenTry> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &StarknetClientId,
        counterparty_client_id: &CosmosClientId,
        counterparty_connection_id: &CosmosConnectionId,
        counterparty_payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        // TODO(rano): use the connection version from the payload
        let connection_version = ConnectionVersion::compatibles()[0].clone();

        let commitment_proof = StateProof {
            proof: counterparty_payload.proof_init.proof_bytes.clone(),
        };

        let proof_height = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let conn_open_init_msg = MsgConnOpenTry {
            client_id_on_a: counterparty_client_id.clone(),
            client_id_on_b: client_id.clone(),
            conn_id_on_a: counterparty_connection_id.clone(),
            prefix_on_a: counterparty_payload.commitment_prefix.into(),
            version_on_a: connection_version,
            proof_conn_end_on_a: commitment_proof,
            proof_height_on_a: proof_height,
            delay_period: Duration::from_secs(0),
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_init_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("conn_open_init"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(ConnectionOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasHeightType
        + HasClientStateType<Counterparty>
        + HasConnectionIdType<Counterparty, ConnectionId = StarknetConnectionId>
        + HasAddressType<Address = StarknetAddress>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasConnectionIdType<Chain, ConnectionId = CosmosConnectionId>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Counterparty, Chain>,
        >,
    Encoding: CanEncode<ViaCairo, MsgConnOpenAck> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_connection_open_ack_message(
        chain: &Chain,
        connection_id: &StarknetConnectionId,
        counterparty_connection_id: &CosmosConnectionId,
        counterparty_payload: ConnectionOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        // TODO(rano): use the connection version from the payload
        let connection_version = ConnectionVersion::compatibles()[0].clone();

        let commitment_proof = StateProof {
            proof: counterparty_payload.proof_try.proof_bytes.clone(),
        };

        let proof_height = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let conn_open_ack_msg = MsgConnOpenAck {
            conn_id_on_a: connection_id.clone(),
            conn_id_on_b: counterparty_connection_id.clone(),
            proof_conn_end_on_b: commitment_proof,
            proof_height_on_b: proof_height,
            version: connection_version,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_ack_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("conn_open_ack"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

#[cgp_provider(ConnectionOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = StarknetConnectionId>
        + HasAddressType<Address = StarknetAddress>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        >,
    Encoding: CanEncode<ViaCairo, MsgConnOpenConfirm> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_connection_open_confirm_message(
        chain: &Chain,
        connection_id: &StarknetConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let commitment_proof = StateProof {
            proof: counterparty_payload.proof_ack.proof_bytes.clone(),
        };

        let proof_height = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let conn_open_confirm_msg = MsgConnOpenConfirm {
            conn_id_on_b: connection_id.clone(),
            proof_conn_end_on_a: commitment_proof,
            proof_height_on_a: proof_height,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_confirm_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *ibc_core_address,
            selector: selector!("conn_open_confirm"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}
