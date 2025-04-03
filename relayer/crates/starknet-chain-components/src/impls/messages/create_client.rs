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
use ibc::core::commitment_types::specs::ProofSpecs;
use starknet::core::types::{Call, Felt};
use starknet::macros::{selector, short_string};

use crate::impls::types::address::StarknetAddress;
use crate::impls::types::message::StarknetMessage;
use crate::impls::utils::array::from_vec_u8_to_be_u32_slice;
use crate::traits::queries::contract_address::CanQueryContractAddress;
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

        let root_slice = from_vec_u8_to_be_u32_slice(root).map_err(Chain::raise_error)?;

        let client_type = short_string!("07-tendermint");

        let client_state = CometClientState {
            latest_height: height,
            trusting_period: payload.client_state.trusting_period,
            unbonding_period: payload.client_state.unbonding_period,
            max_clock_drift: payload.client_state.max_clock_drift,
            status: ClientStatus::Active,
            chain_id: payload.client_state.chain_id,
            proof_specs: ProofSpecs::cosmos(),
        };

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
