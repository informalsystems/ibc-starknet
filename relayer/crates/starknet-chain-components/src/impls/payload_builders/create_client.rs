use core::marker::PhantomData;
use core::time::Duration;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, CreateClientPayloadBuilder,
    CreateClientPayloadBuilderComponent, HasAddressType, HasChainId,
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
    OverrideCreateClientPayloadOptionsComponent, ProvideOverrideCreateClientPayloadOptions,
};
use hermes_core::encoding_components::traits::{CanDecode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_block_verifier::Endpoint as FeederGatewayEndpoint;
use starknet_v14::core::types::StorageProof;

use crate::impls::StarknetAddress;
use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType,
    HasFeederGatewayUrl, HasSelectorType,
};
use crate::types::{
    StarknetChainStatus, StarknetConsensusState, StarknetCreateClientPayload,
    StarknetCreateClientPayloadOptions,
};

pub struct BuildStarknetCreateClientPayload;

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, Encoding> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetCreateClientPayload
where
    Chain: HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = StarknetCreateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasFeederGatewayUrl
        + CanQueryStorageProof<StorageProof = StorageProof>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryChainHeight<Height = u64>
        + HasChainId<ChainId = ChainId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ureq::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: Async + CanDecode<ViaCairo, u64> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &StarknetCreateClientPayloadOptions,
    ) -> Result<StarknetCreateClientPayload, Chain::Error> {
        let height = chain.query_chain_height().await?;

        let block = chain.query_block(&height).await?;

        let storage_proof = chain
            .query_storage_proof(
                &height,
                &chain.query_contract_address(PhantomData).await?,
                &[],
            )
            .await?;

        if block.block_hash != storage_proof.global_roots.block_hash {
            return Err(Chain::raise_error(
                "block hash does not match between block and storage proof",
            ));
        }

        let contract_root = storage_proof
            .contracts_proof
            .contract_leaves_data
            .first()
            .and_then(|leaf| leaf.storage_root)
            .ok_or_else(|| Chain::raise_error("contract root not found in storage proof"))?;

        let root = contract_root.to_bytes_be().to_vec();

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: u64::try_from(block.time.unix_timestamp_nanos())
                .ok()
                .map(Timestamp::from_nanoseconds)
                .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let feeder_endpoint = FeederGatewayEndpoint::new(chain.feeder_gateway_url().as_str());

        let sequencer_public_key = feeder_endpoint
            .get_public_key(Some(height))
            .map_err(Chain::raise_error)?;

        let final_height = {
            let output = chain
                .call_contract(
                    &ibc_core_address,
                    &selector!("get_final_height"),
                    &vec![],
                    Some(&height),
                )
                .await?;

            chain
                .encoding()
                .decode(&output)
                .map_err(Chain::raise_error)?
        };

        Ok(StarknetCreateClientPayload {
            latest_height: Height::new(0, block.height).map_err(Chain::raise_error)?,
            final_height,
            chain_id: chain.chain_id().clone(),
            client_state_wasm_code_hash: create_client_options.wasm_code_hash.into(),
            consensus_state,
            sequencer_public_key: sequencer_public_key.to_bytes_be().to_vec(),
            ibc_contract_address: ibc_core_address.to_bytes_be().to_vec(),
        })
    }
}

pub struct ProvideNoCreateClientMessageOptionsOverride;

#[cgp_provider(OverrideCreateClientPayloadOptionsComponent)]
impl<Chain, Counterparty> ProvideOverrideCreateClientPayloadOptions<Chain, Counterparty>
    for ProvideNoCreateClientMessageOptionsOverride
where
    Chain: HasCreateClientPayloadOptionsType<
        Counterparty,
        CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
    >,
{
    fn override_create_client_payload_options(
        payload_options: &StarknetCreateClientPayloadOptions,
        _new_period: Duration,
    ) -> StarknetCreateClientPayloadOptions {
        payload_options.clone()
    }
}
