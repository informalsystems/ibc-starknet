use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, CreateClientPayloadBuilder,
    CreateClientPayloadBuilderComponent, HasAddressType, HasChainId,
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;
use starknet_v14::core::types::StorageProof;

use crate::impls::StarknetAddress;
use crate::traits::{CanQueryContractAddress, CanQueryStorageProof, HasStarknetProofSigner};
use crate::types::{
    StarknetChainStatus, StarknetConsensusState, StarknetCreateClientPayload,
    StarknetCreateClientPayloadOptions, WasmStarknetConsensusState,
};

pub struct BuildStarknetCreateClientPayload;

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetCreateClientPayload
where
    Chain: HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = StarknetCreateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryStorageProof<StorageProof = StorageProof>
        + HasAddressType<Address = StarknetAddress>
        + CanQueryChainHeight<Height = u64>
        + HasChainId<ChainId = ChainId>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<ClientError>,
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

        let contract_root = storage_proof
            .contracts_proof
            .contract_leaves_data
            .first()
            .and_then(|leaf| leaf.storage_root)
            .unwrap();

        let root = contract_root.to_bytes_be().to_vec();

        let consensus_state = WasmStarknetConsensusState {
            consensus_state: StarknetConsensusState {
                root: root.into(),
                time: u64::try_from(block.time.unix_timestamp_nanos())
                    .ok()
                    .map(Timestamp::from_nanoseconds)
                    .ok_or_else(|| Chain::raise_error("invalid timestamp"))?,
            },
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        Ok(StarknetCreateClientPayload {
            latest_height: Height::new(0, block.height).map_err(Chain::raise_error)?,
            chain_id: chain.chain_id().clone(),
            client_state_wasm_code_hash: create_client_options.wasm_code_hash.into(),
            consensus_state,
            proof_signer_pub_key: chain.proof_signer().public_key.serialize().to_vec(),
            ibc_contract_address: ibc_core_address.to_bytes_be().to_vec(),
        })
    }
}
