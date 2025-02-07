use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_chain_components::traits::types::create_client::HasCreateClientMessageOptionsType;
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::client_id::ClientId;
use crate::types::cosmos::update::CometUpdateHeader;

pub struct BuildUpdateCometClientMessage;

impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCometClientMessage
where
    Chain: HasCreateClientMessageOptionsType<Counterparty>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = Felt>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, CometUpdateHeader>
        + CanEncode<ViaCairo, (ClientId, Vec<Felt>)>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &ClientId,
        counterparty_payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let mut messages = Vec::with_capacity(counterparty_payload.headers.len());

        for header in counterparty_payload.headers {
            let update_header = CometUpdateHeader::from(header);

            let encoding = chain.encoding();

            let contract_address = chain.query_contract_address(PhantomData).await?;

            let raw_header = encoding
                .encode(&update_header)
                .map_err(Chain::raise_error)?;

            let calldata = encoding
                .encode(&(client_id.clone(), raw_header))
                .map_err(Chain::raise_error)?;

            let call = Call {
                to: contract_address,
                selector: selector!("update_client"),
                calldata,
            };

            let message = StarknetMessage::new(call);

            messages.push(message);
        }

        Ok(messages)
    }
}
