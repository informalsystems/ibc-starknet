use cgp::prelude::{CanRaiseError, HasErrorType};
use indexmap::IndexMap;
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::{ContractsProof, Felt, MerkleNode, StorageProof};

use crate::traits::types::storage_proof::HasStorageProofType;

/**
    Try to verify the structure of storage proof according to:
    <https://docs.starknet.io/architecture-and-concepts/network-architecture/starknet-state/>
*/
pub trait CanVerifyStorageProof: HasStorageProofType + HasErrorType {
    fn verify_storage_proof(proof: &Self::StorageProof) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyStorageProof for Chain
where
    Chain: HasStorageProofType<StorageProof = StorageProof> + CanRaiseError<String>,
{
    fn verify_storage_proof(proof: &StorageProof) -> Result<(), Self::Error> {
        Chain::verify_merkle_node_map(
            &proof.classes_proof,
            &vec![proof.global_roots.classes_tree_root],
        )?;

        Chain::verify_merkle_node_map(
            &proof.contracts_proof.nodes,
            &vec![proof.global_roots.contracts_tree_root],
        )?;

        let contract_roots = proof
            .contracts_proof
            .contract_leaves_data
            .iter()
            .flat_map(|leaf| leaf.storage_root.into_iter())
            .collect::<Vec<_>>();

        for storage_entry in proof.contracts_storage_proofs.iter() {
            Chain::verify_merkle_node_map(storage_entry, &contract_roots)?;
        }

        Chain::verify_contracts_proof(&proof.contracts_proof)?;

        Ok(())
    }
}

pub trait CanVerifyContractsProof: HasErrorType {
    fn verify_contracts_proof(contracts_proof: &ContractsProof) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyContractsProof for Chain
where
    Chain: CanRaiseError<String>,
{
    fn verify_contracts_proof(contracts_proof: &ContractsProof) -> Result<(), Self::Error> {
        for contract_leaf in contracts_proof.contract_leaves_data.iter() {
            let storage_root = contract_leaf.storage_root.ok_or_else(|| {
                Chain::raise_error(format!("storage root not found at {contract_leaf:?}"))
            })?;

            let contract_hash = pedersen_hash(
                &pedersen_hash(
                    &pedersen_hash(&contract_leaf.class_hash, &storage_root),
                    &contract_leaf.nonce,
                ),
                &Felt::ZERO,
            );

            let _node = contracts_proof
                .nodes
                .iter()
                .find_map(|(_, node)| match node {
                    MerkleNode::EdgeNode(node) => {
                        if node.child == contract_hash {
                            Some(node)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    Chain::raise_error(format!(
                        "contract hash {} for {:?} not found in contract proof nodes",
                        contract_hash.to_hex_string(),
                        contract_leaf
                    ))
                })?;

            // TODO: Verify that the edge node is a membership proof
        }

        Ok(())
    }
}

pub trait CanVerifyMerkleNodeMap: HasErrorType {
    fn verify_merkle_node_map(
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &Vec<Felt>,
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyMerkleNodeMap for Chain
where
    Chain: CanRaiseError<String>,
{
    fn verify_merkle_node_map(
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &Vec<Felt>,
    ) -> Result<(), Self::Error> {
        for (hash, node) in node_map.iter() {
            Chain::verify_merkle_node(hash, node)?;
        }

        for (hash, node) in node_map.iter() {
            Chain::verify_merkle_node_parent(hash, node_map, roots)?;
        }

        Ok(())
    }
}

/**
   Validates that each node entry is either the child of another entry,
   or is one of the root nodes.
*/
pub trait CanVerifyMerkleNodeParent: HasErrorType {
    fn verify_merkle_node_parent(
        node_hash: &Felt,
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &Vec<Felt>,
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyMerkleNodeParent for Chain
where
    Chain: CanRaiseError<String>,
{
    fn verify_merkle_node_parent(
        hash: &Felt,
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &Vec<Felt>,
    ) -> Result<(), Self::Error> {
        if roots.contains(hash) {
            return Ok(());
        }

        for (parent_hash, parent_node) in node_map.iter() {
            if parent_hash != hash {
                match parent_node {
                    MerkleNode::BinaryNode(parent_node) => {
                        if &parent_node.left == hash || &parent_node.right == hash {
                            return Ok(());
                        }
                    }
                    MerkleNode::EdgeNode(parent_node) => {
                        // FIXME: How to verify that a child value is a node or a storage value?
                        // If the child is a storage value, we should *not* treat it as a valid parent.
                        if &parent_node.child == hash {
                            return Ok(());
                        }
                    }
                }
            }
        }

        Err(Chain::raise_error(format!(
            "failed to find parent node for child node with hash {}",
            hash.to_hex_string()
        )))
    }
}

pub trait CanVerifyMerkleNode: HasErrorType {
    fn verify_merkle_node(node_hash: &Felt, node: &MerkleNode) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyMerkleNode for Chain
where
    Chain: CanRaiseError<String>,
{
    fn verify_merkle_node(node_hash: &Felt, node: &MerkleNode) -> Result<(), Self::Error> {
        match node {
            MerkleNode::BinaryNode(node) => {
                let expected = pedersen_hash(&node.left, &node.right);

                if &expected != node_hash {
                    return Err(Chain::raise_error(format!(
                        "error validating binary node {node:?}. expected hash: {expected}, got: {node_hash}"
                    )));
                }
            }

            MerkleNode::EdgeNode(node) => {
                let expected = pedersen_hash(&node.child, &node.path) + node.length;

                if &expected != node_hash {
                    return Err(Chain::raise_error(format!(
                        "error validating edge node {node:?}. expected hash: {expected}, got: {node_hash}"
                    )));
                }
            }
        }

        Ok(())
    }
}
