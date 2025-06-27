use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanQueryBlock, HasClientStateType, HasHeightType, HasUpdateClientPayloadType,
    UpdateClientPayloadBuilder, UpdateClientPayloadBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanEncode, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet::providers::ProviderError;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanQueryContractAddress, CanQueryStorageProof, HasStarknetClient, HasStarknetProofSigner,
};
use crate::types::{StarknetChainStatus, StarknetConsensusState, StarknetUpdateClientPayload};

pub struct BuildStarknetUpdateClientPayload;

#[cgp_provider(UpdateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryStorageProof<StorageProof = StorageProof>
        + HasStarknetClient
        + CanRaiseAsyncError<&'static str>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: Async + CanEncode<ViaProtobuf, StarknetHeader, Encoded = Vec<u8>>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &u64,
        target_height: &u64,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let block = chain.query_block(target_height).await?;

        // TODO(rano): we actually need to pass the block header along with contract root.
        let storage_proof = chain
            .query_storage_proof(
                target_height,
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

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::from_nanoseconds(
                u64::try_from(block.time.unix_timestamp_nanos()).unwrap(),
            ),
        };

        let height = Height::new(0, *target_height).unwrap();

        let header = StarknetHeader {
            height,
            consensus_state,
        };

        let encoded_header = Chain::default_encoding()
            .encode(&header)
            .map_err(Chain::raise_error)?;

        let signature = chain
            .proof_signer()
            .sign(&encoded_header)
            .map_err(Chain::raise_error)?;

        Ok(StarknetUpdateClientPayload { header, signature })
    }
}
