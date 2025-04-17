use core::fmt::Display;

use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProof {
    pub contracts_proof: ContractsProof,
    pub nodes: Vec<NodeEntry>,
    pub global_roots: GlobalRoots,
}

impl Display for StorageProof {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let proof_json = serde_json::to_string_pretty(self).map_err(|_| core::fmt::Error)?;
        write!(f, "{proof_json}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueNode {
    Branch { left: Felt, right: Felt },
    Leaf { child: Felt, length: u8, path: Felt },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeEntry {
    pub node: ValueNode,
    pub node_hash: Felt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractNode {
    pub class_hash: Felt,
    pub nonce: Felt,
    pub storage_root: Felt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractsProof {
    pub contract_leaves_data: Vec<ContractNode>,
    pub nodes: Vec<NodeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalRoots {
    pub block_hash: Felt,
    pub classes_tree_root: Felt,
    pub contracts_tree_root: Felt,
}
