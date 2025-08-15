use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasClientStateType, HasHeightType, HasUpdateClientPayloadType,
    UpdateClientPayloadBuilder, UpdateClientPayloadBuilderComponent,
};
use hermes_core::encoding_components::traits::{
    CanDecode, CanEncode, HasDefaultEncoding, HasEncodedType, HasEncoding,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet::providers::ProviderError;
use starknet_block_verifier::Endpoint as FeederGatewayEndpoint;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType,
    HasFeederGatewayUrl, HasSelectorType, HasStarknetClient,
};
use crate::types::{StarknetChainStatus, StarknetConsensusState, StarknetUpdateClientPayload};

pub struct BuildStarknetUpdateClientPayload;

#[cgp_provider(UpdateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, ProtoEncoding, CairoEncoding>
    UpdateClientPayloadBuilder<Chain, Counterparty> for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryStorageProof<StorageProof = StorageProof, StorageKey = Felt>
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasStarknetClient
        + HasFeederGatewayUrl
        + CanRaiseAsyncError<&'static str>
        + HasDefaultEncoding<AsBytes, Encoding = ProtoEncoding>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<ureq::Error>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<ProtoEncoding::Error>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    ProtoEncoding: Async + CanEncode<ViaProtobuf, StarknetHeader, Encoded = Vec<u8>>,
    CairoEncoding: Async + CanDecode<ViaCairo, u64> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &u64,
        target_height: &u64,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let block = chain.query_block(target_height).await?;

        let feeder_endpoint_url = chain.feeder_gateway_url();
        let feeder_endpoint = FeederGatewayEndpoint::new(feeder_endpoint_url.as_str());

        let block_header = feeder_endpoint
            .get_block_header(Some(*target_height))
            .map_err(Chain::raise_error)?;

        let block_signature = feeder_endpoint
            .get_signature(Some(*target_height))
            .map_err(Chain::raise_error)?;

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let final_height_key = selector!("final_height");

        let final_height = {
            let output = chain
                .call_contract(
                    &ibc_core_address,
                    &selector!("get_final_height"),
                    &vec![],
                    Some(target_height),
                )
                .await?;

            chain
                .encoding()
                .decode(&output)
                .map_err(Chain::raise_error)?
        };

        let storage_proof = chain
            .query_storage_proof(target_height, &ibc_core_address, &[final_height_key])
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
            time: Timestamp::from_nanoseconds(
                u64::try_from(block.time.unix_timestamp_nanos()).unwrap(),
            ),
        };

        let height = Height::new(0, *target_height).unwrap();

        let header = StarknetHeader {
            block_header,
            final_height,
            block_signature,
            storage_proof,
        };

        let encoded_header = Chain::default_encoding()
            .encode(&header)
            .map_err(Chain::raise_error)?;

        Ok(StarknetUpdateClientPayload { header })
    }
}
