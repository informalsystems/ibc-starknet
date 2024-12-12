use core::marker::PhantomData;
use core::num::ParseIntError;
use core::str::{from_utf8, FromStr, Utf8Error};

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::commitment_prefix::HasCommitmentPrefixType;
use hermes_chain_components::traits::message_builders::connection_handshake::ConnectionOpenTryMessageBuilder;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenTryPayloadType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasClientIdType, HasConnectionIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::connection::ConnectionOpenTryPayload;
use hermes_chain_type_components::traits::types::address::HasAddressType;
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

use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::client_id::ClientId as StarknetClientId;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::connection::{BasePrefix, ConnectionVersion, MsgConnOpenTry};
use crate::types::messages::ibc::packet::StateProof;

pub struct BuildStarknetConnectionHandshakeMessages;

impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetConnectionHandshakeMessages
where
    Chain: HasHeightType
        + HasMessageType<Message = Call>
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
        let commitment_proof = StateProof { proof: vec![] };

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

        let message = Call {
            to: ibc_core_address,
            selector: selector!("conn_open_init"),
            calldata,
        };

        Ok(message)
    }
}
