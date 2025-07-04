use starknet_block_verifier::{Block, Signature, StarknetCryptoEmpty};
use starknet_core::types::{Felt, StorageProof};
use starknet_storage_verifier::validate::validate_storage_proof;
use starknet_storage_verifier::verifier::{
    verify_starknet_contract_proof, verify_starknet_global_contract_root,
    verify_starknet_storage_proof,
};
use sylvia::ctx::{InstantiateCtx, QueryCtx};
use sylvia::cw_std::{Binary, Response, StdError, StdResult};
use sylvia::{contract, serde_json};

use crate::types::ContractResponse;

pub struct StarknetLightClientLibraryContract {}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[contract]
impl StarknetLightClientLibraryContract {
    pub const fn new() -> Self {
        Self {}
    }

    #[sv::msg(instantiate)]
    pub fn instantiate(&self, ctx: InstantiateCtx<'_>) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[sv::msg(query)]
    pub fn validate_block_header(
        &self,
        ctx: QueryCtx<'_>,
        header: Binary,
        signature: Binary,
        pub_key: Binary,
    ) -> StdResult<ContractResponse> {
        let header: Block = serde_json::from_slice(&header)
            .map_err(|e| StdError::generic_err(format!("Invalid block header: {e}")))?;

        let signature: Signature = serde_json::from_slice(&signature)
            .map_err(|e| StdError::generic_err(format!("Invalid signature: {e}")))?;

        let pub_key: Felt = serde_json::from_slice(&pub_key)
            .map_err(|e| StdError::generic_err(format!("Invalid public key: {e}")))?;

        header
            .verify_signature::<StarknetCryptoEmpty>(&signature, &pub_key)
            .map_err(|e| StdError::generic_err(format!("Block header verification failed: {e}")))?;

        Ok(ContractResponse::StateRoot(
            header.state_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn validate_storage_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: Binary,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_slice(&storage_proof)
            .map_err(|e| StdError::generic_err(format!("Invalid storage proof: {e}")))?;

        validate_storage_proof::<StarknetCryptoEmpty>(&storage_proof)
            .map_err(|e| StdError::generic_err(format!("Storage proof validation failed: {e}")))?;

        Ok(ContractResponse::ValidStorageProof)
    }

    #[sv::msg(query)]
    pub fn verify_starknet_global_contract_root(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: Binary,
        state_root: Binary,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_slice(&storage_proof)
            .map_err(|e| StdError::generic_err(format!("Invalid storage proof: {e}")))?;
        let state_root: Felt = serde_json::from_slice(&state_root)
            .map_err(|e| StdError::generic_err(format!("Invalid state root: {e}")))?;

        let global_contract_trie_root =
            verify_starknet_global_contract_root(&storage_proof, state_root).map_err(|e| {
                StdError::generic_err(format!("Global contract root verification failed: {e}"))
            })?;

        Ok(ContractResponse::GlobalContractTrieRoot(
            global_contract_trie_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn verify_starknet_contract_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: Binary,
        global_contract_trie_root: Binary,
        contract_address: Binary,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_slice(&storage_proof)
            .map_err(|e| StdError::generic_err(format!("Invalid storage proof: {e}")))?;
        let global_contract_trie_root: Felt = serde_json::from_slice(&global_contract_trie_root)
            .map_err(|e| {
                StdError::generic_err(format!("Invalid global contract trie root: {e}"))
            })?;
        let contract_address: Felt = serde_json::from_slice(&contract_address)
            .map_err(|e| StdError::generic_err(format!("Invalid contract address: {e}")))?;

        let contract_root = verify_starknet_contract_proof(
            &storage_proof,
            global_contract_trie_root,
            contract_address,
        )
        .map_err(|e| StdError::generic_err(format!("Contract root verification failed: {e}")))?;

        Ok(ContractResponse::ContractRoot(
            contract_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn verify_starknet_storage_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: Binary,
        contract_root: Binary,
        path: Binary,
        value: Binary,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_slice(&storage_proof)
            .map_err(|e| StdError::generic_err(format!("Invalid storage proof: {e}")))?;
        let contract_root: Felt = serde_json::from_slice(&contract_root)
            .map_err(|e| StdError::generic_err(format!("Invalid contract root: {e}")))?;
        let path: Felt = serde_json::from_slice(&path)
            .map_err(|e| StdError::generic_err(format!("Invalid path: {e}")))?;
        let value: Felt = serde_json::from_slice(&value)
            .map_err(|e| StdError::generic_err(format!("Invalid value: {e}")))?;

        verify_starknet_storage_proof(&storage_proof, contract_root, path, value)
            .map_err(|e| StdError::generic_err(format!("Failed to verify storage proof: {e}")))?;

        Ok(ContractResponse::CorrectStorageProof)
    }
}
