use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, ChannelEndQuerier, ChannelEndQuerierComponent,
    ChannelEndWithProofsQuerier, ChannelEndWithProofsQuerierComponent, HasChannelEndType,
    HasChannelIdType, HasCommitmentProofType, HasHeightType, HasIbcCommitmentPrefix, HasPortIdType,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::host::types::path::{ChannelEndPath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStarknetProofSigner, HasStorageKeyType, HasStorageProofType,
};
use crate::types::{ChannelEnd, ChannelId, PortId, StarknetChainStatus, StarknetCommitmentProof};

pub struct QueryChannelEndFromStarknet;

#[cgp_provider(ChannelEndQuerierComponent)]
impl<Chain, Counterparty, Encoding> ChannelEndQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<Chain::ChannelEnd, Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("channel_end"),
                &calldata,
                Some(height),
            )
            .await?;

        encoding.decode(&output).map_err(Chain::raise_error)
    }
}

#[cgp_provider(ChannelEndWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryChannelEndFromStarknet
where
    Chain: HasHeightType<Height = u64>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanQueryChainHeight
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, Product![PortId, ChannelId]>
        + CanDecode<ViaCairo, ChannelEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_channel_end_with_proofs(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<(Chain::ChannelEnd, Chain::CommitmentProof), Chain::Error> {
        // TODO(rano): how to query at a specific height?

        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone()])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("channel_end"),
                &calldata,
                Some(height),
            )
            .await?;

        let channel_end = encoding.decode(&output).map_err(Chain::raise_error)?;

        let block = chain.query_block(height).await?;

        let ibc_path = Path::ChannelEnd(ChannelEndPath::new(port_id, channel_id));

        let felt_path: Felt = ibc_path_to_storage_key(&StarknetCryptoLib, ibc_path);

        // key == path
        let storage_proof: StorageProof = chain
            .query_storage_proof(height, &contract_address, &[felt_path])
            .await?;

        let storage_proof_bytes = serde_json::to_vec(&storage_proof).map_err(Chain::raise_error)?;

        let dummy_proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: storage_proof_bytes,
        };

        Ok((channel_end, dummy_proof))
    }
}
