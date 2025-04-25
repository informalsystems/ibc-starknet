use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::update_client::{
    UpdateClientMessageBuilder, UpdateClientMessageBuilderComponent,
};
use hermes_chain_components::traits::types::create_client::HasCreateClientMessageOptionsType;
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;
use ibc_proto::Protobuf;
use starknet::core::types::{ByteArray, Felt};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::contract_address::CanQueryContractAddress;
use crate::types::client_id::ClientId;
use crate::types::cosmos::update::ClientMessage;

pub struct BuildUpdateCometClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCometClientMessage
where
    Chain: HasCreateClientMessageOptionsType<Counterparty>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, ClientMessage>
        + CanEncode<ViaCairo, ByteArray>
        + CanEncode<ViaCairo, Product![ClientId, Vec<Felt>]>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &ClientId,
        counterparty_payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let mut messages = Vec::with_capacity(counterparty_payload.headers.len());

        for header in counterparty_payload.headers {
            let encoding = chain.encoding();

            let contract_address = chain.query_contract_address(PhantomData).await?;

            // We are not passing the Cairo serialization of the Client Header here.
            // As it has a lot of hash fields as `Vec<u8>`. In the Cairo serialization,
            // they are be encoded as `Array<felt252>` wasting a lot of encoding data space.
            //
            // So, we encode the Header as Protobuf bytes and then encode those bytes as
            // Cairo `ByteArray` which has more succinct `Vec<u8>` representation.

            let protobuf_bytes = Protobuf::<RawHeader>::encode_vec(header.clone());

            let protobuf_byte_array: ByteArray = protobuf_bytes.into();

            let raw_header = encoding
                .encode(&protobuf_byte_array)
                .map_err(Chain::raise_error)?;

            let client_message_felts = encoding
                .encode(&ClientMessage::Update(raw_header))
                .map_err(Chain::raise_error)?;

            let calldata = encoding
                .encode(&product![client_id.clone(), client_message_felts])
                .map_err(Chain::raise_error)?;

            let message =
                StarknetMessage::new(*contract_address, selector!("update_client"), calldata);

            messages.push(message);
        }

        Ok(messages)
    }
}
