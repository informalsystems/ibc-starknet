use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::create_client::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent,
};
use hermes_chain_components::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::{selector, short_string};

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::cosmos::client_state::{ClientStatus, CometClientState};
use crate::types::cosmos::consensus_state::CometConsensusState;
use crate::types::cosmos::height::Height;

pub struct BuildCreateCometClientMessage;

#[cgp_provider(CreateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCometClientMessage
where
    Chain: HasCreateClientMessageOptionsType<Counterparty>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<core::num::TryFromIntError>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, CometClientState>
        + CanEncode<ViaCairo, CometConsensusState>
        + CanEncode<ViaCairo, Product![Felt, Vec<Felt>, Vec<Felt>]>,
{
    async fn build_create_client_message(
        chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        payload: CosmosCreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let height = Height {
            revision_number: payload.client_state.latest_height.revision_number(),
            revision_height: payload.client_state.latest_height.revision_height(),
        };

        let root = payload.consensus_state.root.into_vec();

        let client_type = short_string!("07-tendermint");

        let client_state = CometClientState {
            latest_height: height,
            trusting_period: payload.client_state.trusting_period,
            unbonding_period: payload.client_state.unbonding_period,
            max_clock_drift: payload.client_state.max_clock_drift,
            status: ClientStatus::Active,
            chain_id: payload.client_state.chain_id,
        };

        // Convert Vec<u8> to a slice of 8 u32 values.
        let mut root_u32: Vec<u32> = Vec::new();
        for chunk in root.chunks_exact(4) {
            let chunk_u8: [u8; 4] = chunk.try_into().expect("Chunks must have 4 elements");
            let value_u32 = u32::from_le_bytes(chunk_u8);
            root_u32.push(value_u32);
        }
        let root_slice: [u32; 8] = root_u32.try_into().map_err(|e| {
            Chain::raise_error(format!("failed to convert Vec<u32> to [u32; 8]: {e:?}"))
        })?;

        let consensus_state = CometConsensusState {
            timestamp: u64::try_from(payload.consensus_state.timestamp.unix_timestamp_nanos())
                .map_err(Chain::raise_error)?,
            root: root_slice,
        };

        let raw_client_state = encoding.encode(&client_state).map_err(Chain::raise_error)?;
        let raw_consensus_state = encoding
            .encode(&consensus_state)
            .map_err(Chain::raise_error)?;

        let calldata = encoding
            .encode(&product![
                client_type,
                raw_client_state,
                raw_consensus_state
            ])
            .map_err(Chain::raise_error)?;

        let call = Call {
            to: *contract_address,
            selector: selector!("create_client"),
            calldata,
        };

        let message = StarknetMessage::new(call);

        Ok(message)
    }
}
