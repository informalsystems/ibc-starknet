use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryClientStateWithLatestHeight, HasAddressType, HasChainId, HasClientIdType,
    HasClientStateType, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::{LevelDebug, LevelWarn};
use hermes_prelude::*;
use ibc::core::host::types::error::DecodingError;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::Protobuf;
use prost_types::Any;
use starknet::core::types::{ByteArray, Felt, U256};
use starknet::macros::selector;
use tendermint_proto::Error as TendermintProtoError;

use crate::impls::{
    comet_signature_hints, CosmosStarknetMisbehaviour, StarknetAddress, StarknetMessage,
};
use crate::traits::{CanQueryContractAddress, HasEd25519AttestatorAddresses};
use crate::types::{ClientMessage, CometClientState};

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, EncodingError> MisbehaviourMessageBuilder<Chain, Counterparty>
    for StarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = Any>
        + HasChainId
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasEncoding<AsFelt>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEd25519AttestatorAddresses
        + CanLog<LevelWarn>
        + CanLog<LevelDebug>
        + HasMessageType
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<EncodingError>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<TendermintProtoError>,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState>,
    Chain::Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Product![U256, U256, U256, Vec<u8>], Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Product![Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Vec<Product![Felt, Felt, Felt]>>
        + CanEncode<ViaCairo, Vec<Vec<Felt>>>
        + CanEncode<ViaCairo, (Vec<Vec<Felt>>, Vec<Vec<Felt>>)>
        + CanEncode<ViaCairo, Product![ClientMessage, Vec<Felt>]>
        + CanEncode<ViaCairo, ByteArray>
        + CanEncode<ViaCairo, Product![ClientId, Vec<Felt>]>
        + HasAsyncErrorType<Error = EncodingError>,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        let decoded_evidence: CosmosStarknetMisbehaviour =
            Protobuf::decode(&*evidence.value).map_err(Chain::raise_error)?;

        let encoding = chain.encoding();
        let contract_address = chain.query_contract_address(PhantomData).await?;

        // We are not passing the Cairo serialization of the Client Header here.
        // As it has a lot of hash fields as `Vec<u8>`. In the Cairo serialization,
        // they are be encoded as `Array<felt252>` wasting a lot of encoding data space.
        //
        // So, we encode the Header as Protobuf bytes and then encode those bytes as
        // Cairo `ByteArray` which has more succinct `Vec<u8>` representation.

        let ed25519_attestator_addresses = chain.ed25519_attestator_addresses();

        let client_state = chain
            .query_client_state_with_latest_height(PhantomData, client_id)
            .await?;

        let attestator_keys = client_state.attestator_keys;

        if attestator_keys.len() != ed25519_attestator_addresses.len() {
            return Err(Chain::raise_error(
                "Attestator keys and addresses length mismatch",
            ));
        }

        let attestator_quorum_percentage = client_state.attestator_quorum_percentage;

        let signature_hint_1 = comet_signature_hints(
            chain,
            &decoded_evidence
                .evidence_1
                .clone()
                .try_into()
                .map_err(Chain::raise_error)?,
            encoding,
            ed25519_attestator_addresses,
            attestator_quorum_percentage,
        )
        .await;

        let signature_hint_2 = comet_signature_hints(
            chain,
            &decoded_evidence
                .evidence_2
                .clone()
                .try_into()
                .map_err(Chain::raise_error)?,
            encoding,
            ed25519_attestator_addresses,
            attestator_quorum_percentage,
        )
        .await;

        let serialized_signature_hints = encoding
            .encode(&(signature_hint_1, signature_hint_2))
            .map_err(Chain::raise_error)?;

        let protobuf_bytes = Protobuf::<Misbehaviour>::encode_vec(decoded_evidence.clone());

        let protobuf_byte_array: ByteArray = protobuf_bytes.into();

        let raw_misbehaviour = encoding
            .encode(&protobuf_byte_array)
            .map_err(Chain::raise_error)?;

        let client_message_with_hints = product![
            ClientMessage::Misbehaviour(raw_misbehaviour),
            serialized_signature_hints
        ];

        let client_message_felts = encoding
            .encode(&client_message_with_hints)
            .map_err(Chain::raise_error)?;

        let calldata = encoding
            .encode(&product![client_id.clone(), client_message_felts])
            .map_err(Chain::raise_error)?;

        Ok(StarknetMessage::new(
            *contract_address,
            selector!("update_client"),
            calldata,
        ))
    }
}
