use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    HasAddressType, HasChainId, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelWarn;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::Protobuf;
use starknet::core::types::{ByteArray, Felt};
use starknet::macros::selector;

use crate::impls::{StarknetAddress, StarknetMessage, StarknetMisbehaviour};
use crate::traits::CanQueryContractAddress;
use crate::types::ClientMessage;

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, EncodingError> MisbehaviourMessageBuilder<Chain, Counterparty>
    for StarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType<Evidence = StarknetMisbehaviour>
        + HasChainId
        + HasEncoding<AsFelt>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanLog<LevelWarn>
        + HasMessageType
        + CanRaiseAsyncError<EncodingError>,
    Chain::Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, Product![ClientMessage, Vec<Felt>]>
        + CanEncode<ViaCairo, ByteArray>
        + CanEncode<ViaCairo, Product![ClientId, Vec<Felt>]>
        + HasAsyncErrorType<Error = EncodingError>,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        // TODO
        chain
            .log(
                &format!("StarknetMisbehaviourMessageBuilder {}", chain.chain_id()),
                &LevelWarn,
            )
            .await;

        let encoding = chain.encoding();
        let contract_address = chain.query_contract_address(PhantomData).await?;

        // We are not passing the Cairo serialization of the Client Header here.
        // As it has a lot of hash fields as `Vec<u8>`. In the Cairo serialization,
        // they are be encoded as `Array<felt252>` wasting a lot of encoding data space.
        //
        // So, we encode the Header as Protobuf bytes and then encode those bytes as
        // Cairo `ByteArray` which has more succinct `Vec<u8>` representation.

        let protobuf_bytes = Protobuf::<Misbehaviour>::encode_vec(evidence.clone());

        let protobuf_byte_array: ByteArray = protobuf_bytes.into();

        let raw_misbehaviour = encoding
            .encode(&protobuf_byte_array)
            .map_err(Chain::raise_error)?;

        let client_message_with_hints =
            product![ClientMessage::Misbehaviour(raw_misbehaviour), vec![]];

        let client_message_felts = encoding
            .encode(&client_message_with_hints)
            .map_err(Chain::raise_error)?;

        let calldata = encoding
            .encode(&product![evidence.client_id.clone(), client_message_felts])
            .map_err(Chain::raise_error)?;

        Ok(StarknetMessage::new(
            *contract_address,
            selector!("update_client"),
            calldata,
        ))
    }
}
