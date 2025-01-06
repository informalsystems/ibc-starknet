use core::marker::PhantomData;
use core::num::ParseIntError;
use core::str::{from_utf8, FromStr, Utf8Error};

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
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
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{
    ClientId as CosmosClientId, ConnectionId as CosmosConnectionId,
};
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::connection::{
    BasePrefix, ConnectionVersion, MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit,
    MsgConnOpenTry,
};
use crate::types::messages::ibc::packet::StateProof;

pub struct BuildStarknetConnectionHandshakeMessages;

impl<Chain, Counterparty, Encoding> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasClientIdType<Counterparty, ClientId = StarknetClientId>
        + HasAddressType<Address = Felt>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseError<&'static str>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<ParseIntError>
        + CanRaiseError<serde_json::Error>
        + CanRaiseError<Utf8Error>,
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
        let counterparty_client_id_as_cairo = {
            let counterparty_client_id_str = counterparty_client_id.to_string();
            let (client_type, sequence_str) = counterparty_client_id_str
                .rsplit_once('-')
                .ok_or_else(|| Chain::raise_error("malformatted client id"))?;

            Chain::ClientId {
                client_type: Felt::from_bytes_be_slice(client_type.as_bytes()),
                sequence: sequence_str.parse().map_err(Chain::raise_error)?,
            }
        };

        // TODO(rano): dirty hack; make fields public at ibc-rs and tidy it up

        let json_connection_version =
            serde_json::to_value(&init_connection_options.connection_version)
                .map_err(Chain::raise_error)?;

        let identifier = json_connection_version
            .pointer("/identifier")
            .ok_or_else(|| Chain::raise_error("connection version does not have a features field"))?
            .as_str()
            .ok_or_else(|| Chain::raise_error("connection version identifier is not a string"))?;

        let features = json_connection_version
            .pointer("/features")
            .ok_or_else(|| Chain::raise_error("connection version does not have a features field"))?
            .as_array()
            .ok_or_else(|| Chain::raise_error("connection version features is not an array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| Chain::raise_error("connection version feature is not a string"))
            })
            .collect::<Result<Vec<&str>, Chain::Error>>()?;

        if features.len() != 2 {
            return Err(Chain::raise_error(
                "connection version features must have 2 elements",
            ));
        }

        let cairo_connection_version = ConnectionVersion {
            identifier: identifier.to_string(),
            features: [features[0].to_string(), features[1].to_string()],
        };

        let base_prefix = BasePrefix {
            prefix: from_utf8(&counterparty_payload.commitment_prefix)
                .map_err(Chain::raise_error)?
                .to_owned(),
        };

        let conn_open_init_msg = MsgConnOpenInit {
            client_id_on_a: client_id.clone(),
            client_id_on_b: counterparty_client_id_as_cairo.clone(),
            prefix_on_b: base_prefix,
            version: cairo_connection_version,
            delay_period: init_connection_options.delay_period.as_secs(),
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_init_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: ibc_core_address,
            selector: selector!("conn_open_init"),
            calldata,
        };

        let message = StarknetMessage::new(call);

        Ok(message)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasHeightType
        + HasMessageType<Message = StarknetMessage>
        + HasClientIdType<Counterparty, ClientId = StarknetClientId>
        + HasClientStateType<Counterparty>
        + HasAddressType<Address = Felt>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseError<ParseIntError>
        + CanRaiseError<Utf8Error>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<&'static str>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType
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
        payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        // FIXME: Cairo IBC should accept counterparty client ID as string value
        let cosmos_client_id_as_cairo = {
            let cosmos_client_id_str = counterparty_client_id.to_string();
            let (client_type, sequence_str) = cosmos_client_id_str
                .rsplit_once('-')
                .ok_or_else(|| Chain::raise_error("malformatted client id"))?;

            StarknetClientId {
                client_type: Felt::from_bytes_be_slice(client_type.as_bytes()),
                sequence: u64::from_str(sequence_str).map_err(Chain::raise_error)?,
            }
        };

        // FIXME: Cairo IBC should use bytes for commitment prefix
        let commitment_prefix =
            from_utf8(&payload.commitment_prefix).map_err(Chain::raise_error)?;

        // FIXME: Use the connection version from the payload
        let connection_version = ConnectionVersion {
            identifier: "1".into(),
            features: ["ORDER_ORDERED".into(), "ORDER_UNORDERED".into()],
        };

        // FIXME: commitment proof should be in the ByteArray format, not Vec<Felt>
        let commitment_proof = StateProof {
            proof: vec![Felt::ONE],
        };

        let proof_height = CairoHeight {
            revision_number: payload.update_height.revision_number(),
            revision_height: payload.update_height.revision_height(),
        };

        let conn_open_init_msg: MsgConnOpenTry = MsgConnOpenTry {
            client_id_on_a: client_id.clone(),
            client_id_on_b: cosmos_client_id_as_cairo.clone(),
            conn_id_on_a: StarknetConnectionId {
                connection_id: counterparty_connection_id.to_string(),
            },
            prefix_on_a: BasePrefix {
                prefix: commitment_prefix.into(),
            },
            version_on_a: connection_version,
            proof_conn_end_on_a: commitment_proof,
            proof_height_on_a: proof_height,
            delay_period: 0,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&conn_open_init_msg)
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: ibc_core_address,
            selector: selector!("conn_open_init"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasHeightType
        + HasClientStateType<Counterparty>
        + HasConnectionIdType<Counterparty, ConnectionId = StarknetConnectionId>
        + HasAddressType<Address = Felt>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType
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
        let counterparty_connection_id_as_cairo = StarknetConnectionId {
            connection_id: counterparty_connection_id.to_string(),
        };

        // TODO(rano): use the connection version from the payload
        let connection_version = ConnectionVersion {
            identifier: "1".into(),
            features: ["ORDER_ORDERED".into(), "ORDER_UNORDERED".into()],
        };

        // FIXME: commitment proof should be in the ByteArray format, not Vec<Felt>
        // TODO(rano): submitting dummy proofs
        // proof can't be empty
        let commitment_proof = StateProof {
            proof: vec![Felt::ONE],
        };

        let proof_height = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let conn_open_ack_msg = MsgConnOpenAck {
            conn_id_on_a: connection_id.clone(),
            conn_id_on_b: counterparty_connection_id_as_cairo,
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
            to: ibc_core_address,
            selector: selector!("conn_open_ack"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = StarknetConnectionId>
        + HasAddressType<Address = Felt>
        + HasMessageType<Message = StarknetMessage>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasHeightType<Height = Height>
        + HasCommitmentProofType
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
        // FIXME: commitment proof should be in the ByteArray format, not Vec<Felt>
        // TODO(rano): submitting dummy proofs
        // proof can't be empty
        let commitment_proof = StateProof {
            proof: vec![Felt::ONE],
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
            to: ibc_core_address,
            selector: selector!("conn_open_confirm"),
            calldata,
        };

        let message =
            StarknetMessage::new(call).with_counterparty_height(counterparty_payload.update_height);

        Ok(message)
    }
}
