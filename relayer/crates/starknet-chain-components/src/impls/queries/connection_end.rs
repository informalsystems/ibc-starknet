use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, ConnectionEndQuerier, ConnectionEndQuerierComponent,
    ConnectionEndWithProofsQuerier, ConnectionEndWithProofsQuerierComponent,
    HasCommitmentProofType, HasConnectionEndType, HasConnectionIdType, HasHeightType,
    HasIbcCommitmentPrefix,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::host::types::path::{ConnectionPath, Path};
use starknet::core::types::{Felt, StorageProof};
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStorageKeyType, HasStorageProofType,
};
use crate::types::{ConnectionEnd, ConnectionId, StarknetChainStatus, StarknetCommitmentProof};

pub struct QueryConnectionEndFromStarknet;

#[cgp_provider(ConnectionEndQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConnectionEndQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<Chain::ConnectionEnd, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("connection_end"),
                &calldata,
                Some(height),
            )
            .await?;

        encoding.decode(&output).map_err(Chain::raise_error)
    }
}

#[cgp_provider(ConnectionEndWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryConnectionEndFromStarknet
where
    Chain: HasHeightType<Height = u64>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: CanEncode<ViaCairo, ConnectionId>
        + CanDecode<ViaCairo, ConnectionEnd>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_connection_end_with_proofs(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<(Chain::ConnectionEnd, Chain::CommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding.encode(connection_id).map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("connection_end"),
                &calldata,
                Some(height),
            )
            .await?;

        let connection_end = encoding.decode(&output).map_err(Chain::raise_error)?;

        let block = chain.query_block(height).await?;

        let ibc_path = Path::Connection(ConnectionPath::new(connection_id));

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

        Ok((connection_end, dummy_proof))
    }
}
