use cosmwasm_std::{Response, StdResult};
use starknet_block_verifier::{Block, Signature};
use starknet_core::types::Felt;
use starknet_core::types::StorageProof;
use starknet_storage_verifier::validate::validate_storage_proof;
use starknet_storage_verifier::verifier::verify_starknet_global_contract_root;
use sylvia::ctx::{InstantiateCtx, QueryCtx};
use sylvia::{contract, serde_json};

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ContractResponse {
    StateRoot(String),
    ValidStorageProof,
    GlobalContractTrieRoot(String),
    ContractRoot(String),
    CorrectStorageProof,
}

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
        header: String,
        signature: String,
        pub_key: String,
    ) -> StdResult<ContractResponse> {
        let header: Block = serde_json::from_str(&header).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid block header: {e}"))
        })?;

        let signature: Signature = serde_json::from_str(&signature)
            .map_err(|e| cosmwasm_std::StdError::generic_err(format!("Invalid signature: {e}")))?;

        let pub_key: Felt = serde_json::from_str(&pub_key)
            .map_err(|e| cosmwasm_std::StdError::generic_err(format!("Invalid public key: {e}")))?;

        header.verify_signature(&signature, &pub_key).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Block header validation failed: {e}"))
        })?;

        Ok(ContractResponse::StateRoot(
            header.state_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn validate_storage_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: String,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_str(&storage_proof).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid storage proof: {e}"))
        })?;
        Ok(ContractResponse::ValidStorageProof)
    }

    #[sv::msg(query)]
    pub fn verify_starknet_global_contract_root(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: String,
        state_root: String,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_str(&storage_proof).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid storage proof: {e}"))
        })?;
        let state_root: Felt = serde_json::from_str(&state_root)
            .map_err(|e| cosmwasm_std::StdError::generic_err(format!("Invalid state root: {e}")))?;

        let global_contract_trie_root =
            verify_starknet_global_contract_root(&storage_proof, state_root).map_err(|e| {
                cosmwasm_std::StdError::generic_err(format!("Verification failed: {e}"))
            })?;

        Ok(ContractResponse::GlobalContractTrieRoot(
            global_contract_trie_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn verify_starknet_contract_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: String,
        global_contract_trie_root: String,
        contract_address: String,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_str(&storage_proof).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid storage proof: {e}"))
        })?;
        let global_contract_trie_root: Felt = serde_json::from_str(&global_contract_trie_root)
            .map_err(|e| {
                cosmwasm_std::StdError::generic_err(format!(
                    "Invalid global contract trie root: {e}"
                ))
            })?;
        let contract_address: Felt = serde_json::from_str(&contract_address).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid contract address: {e}"))
        })?;

        let contract_root =
            verify_starknet_global_contract_root(&storage_proof, global_contract_trie_root)
                .map_err(|e| {
                    cosmwasm_std::StdError::generic_err(format!("Verification failed: {e}"))
                })?;

        Ok(ContractResponse::ContractRoot(
            contract_root.to_fixed_hex_string(),
        ))
    }

    #[sv::msg(query)]
    pub fn verify_starknet_storage_proof(
        &self,
        ctx: QueryCtx<'_>,
        storage_proof: String,
        contract_root: String,
        path: String,
        value: String,
    ) -> StdResult<ContractResponse> {
        let storage_proof: StorageProof = serde_json::from_str(&storage_proof).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid storage proof: {e}"))
        })?;
        let contract_root: Felt = serde_json::from_str(&contract_root).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Invalid contract root: {e}"))
        })?;
        let path: Felt = serde_json::from_str(&path)
            .map_err(|e| cosmwasm_std::StdError::generic_err(format!("Invalid path: {e}")))?;
        let value: Felt = serde_json::from_str(&value)
            .map_err(|e| cosmwasm_std::StdError::generic_err(format!("Invalid value: {e}")))?;

        validate_storage_proof(&storage_proof).map_err(|e| {
            cosmwasm_std::StdError::generic_err(format!("Storage proof validation failed: {e}"))
        })?;

        Ok(ContractResponse::CorrectStorageProof)
    }
}
