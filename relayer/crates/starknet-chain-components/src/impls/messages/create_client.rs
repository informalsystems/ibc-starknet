use core::marker::PhantomData;

use attestator::get_public_key;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType, HasMessageType,
};
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::encoding_components::traits::{CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::types::CosmosCreateClientPayload;
use hermes_prelude::*;
use starknet::core::types::Felt;
use starknet::macros::{selector, short_string};

use crate::impls::{from_vec_u8_to_be_u32_slice, StarknetAddress, StarknetMessage};
use crate::traits::CanQueryContractAddress;
use crate::types::{ClientStatus, CometClientState, CometConsensusState, Height};

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

        let addrs = ["http://localhost:1234"];

        let attestator_keys = addrs.into_iter().map(get_public_key).collect();

        let client_state = CometClientState {
            chain_id: payload.client_state.chain_id,
            latest_height: height,
            trusting_period: payload.client_state.trusting_period,
            unbonding_period: payload.client_state.unbonding_period,
            max_clock_drift: payload.client_state.max_clock_drift,
            trust_level: payload.client_state.trust_level,
            status: ClientStatus::Active,
            proof_specs: payload.client_state.proof_specs,
            upgrade_path: payload.client_state.upgrade_path,
            // FIXME(rano): query attestators' endpoint `/public_key`
            attestator_keys,
        };

        let consensus_state = CometConsensusState {
            timestamp: u64::try_from(payload.consensus_state.timestamp.unix_timestamp_nanos())
                .map_err(Chain::raise_error)?,
            root: root_slice,
            next_validators_hash: payload.consensus_state.next_validators_hash.into(),
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

        let message = StarknetMessage::new(*contract_address, selector!("create_client"), calldata);

        Ok(message)
    }
}
