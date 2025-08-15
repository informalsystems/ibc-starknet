use core::marker::PhantomData;

use cgp::extra::runtime::HasRuntime;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasClientStateType, HasEvidenceType, HasHeightType, HasUpdateClientEvent,
    MisbehaviourChecker, MisbehaviourCheckerComponent,
};
use hermes_core::encoding_components::traits::{
    CanConvert, CanDecode, HasDefaultEncoding, HasEncodedType, HasEncoding,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::runtime_components::traits::CanSleep;
use hermes_cosmos_core::chain_components::types::CosmosUpdateClientEvent;
use hermes_prelude::*;
use ibc_client_starknet_types::header::StarknetHeader;
use ibc_client_starknet_types::misbehaviour::StarknetMisbehaviour;
use prost_types::Any;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet::providers::Provider;
use starknet_block_verifier::Endpoint as FeederGatewayEndpoint;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType,
    HasFeederGatewayUrl, HasSelectorType, HasStarknetClient,
};
use crate::types::StarknetChainStatus;

#[cgp_new_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty, ProtoEncoding, CairoEncoding> MisbehaviourChecker<Chain, Counterparty>
    for CheckStarknetMisbehaviour
where
    Chain: HasClientStateType<Counterparty>
        + HasRuntime
        + HasDefaultEncoding<AsBytes, Encoding = ProtoEncoding>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryStorageProof<StorageProof = StorageProof>
        + HasFeederGatewayUrl
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasHeightType<Height = u64>
        + HasStarknetClient<Client: Provider>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanRaiseAsyncError<ureq::Error>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + CanRaiseAsyncError<ProtoEncoding::Error>,
    Counterparty: HasUpdateClientEvent<UpdateClientEvent = CosmosUpdateClientEvent>
        + HasEvidenceType<Evidence = Any>
        + Async,
    ProtoEncoding: Async
        + CanConvert<Any, StarknetHeader>
        + CanConvert<StarknetMisbehaviour, Any>
        + HasAsyncErrorType,
    CairoEncoding:
        Async + CanDecode<ViaCairo, u64> + HasEncodedType<Encoded = Vec<Felt>> + HasAsyncErrorType,
    Chain::Runtime: CanSleep,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        let encoding = Chain::default_encoding();

        let header: StarknetHeader = encoding
            .convert(&update_client_event.header)
            .map_err(Chain::raise_error)?;

        let target_height = header.block_header.block_number;
        let hash = header.block_header.block_hash;
        let parent_hash = header.block_header.parent_block_hash;

        chain
            .runtime()
            .sleep(core::time::Duration::from_secs(1))
            .await;

        let trusted_block = chain.query_block(&target_height).await?;

        let feeder_endpoint_url = chain.feeder_gateway_url();
        let feeder_endpoint = FeederGatewayEndpoint::new(feeder_endpoint_url.as_str());

        let block_header = feeder_endpoint
            .get_block_header(Some(target_height))
            .map_err(Chain::raise_error)?;

        let block_signature = feeder_endpoint
            .get_signature(Some(target_height))
            .map_err(Chain::raise_error)?;

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let storage_proof = chain
            .query_storage_proof(&target_height, &ibc_core_address, &[])
            .await?;

        let final_height = {
            let output = chain
                .call_contract(
                    &ibc_core_address,
                    &selector!("get_final_height"),
                    &vec![],
                    Some(&target_height),
                )
                .await?;

            chain
                .encoding()
                .decode(&output)
                .map_err(Chain::raise_error)?
        };

        let trusted_header = StarknetHeader {
            block_header,
            block_signature,
            final_height,
            storage_proof,
        };

        if trusted_block.block_hash != header.block_header.block_hash {
            let misbehaviour = StarknetMisbehaviour {
                client_id: update_client_event.client_id.clone(),
                header_1: trusted_header,
                header_2: header,
            };
            let misbehaviour_any: Any = encoding
                .convert(&misbehaviour)
                .map_err(Chain::raise_error)?;
            #[allow(deprecated)]
            return Ok(Some(misbehaviour_any));
        }

        Ok(None)
    }
}
