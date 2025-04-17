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
        Chain::verify_merkle_node_map(&proof.classes_proof)?;
        Chain::verify_merkle_node_map(&proof.contracts_proof.nodes)?;

        for storage_entry in proof.contracts_storage_proofs.iter() {
            Chain::verify_merkle_node_map(storage_entry)?;
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
    fn verify_merkle_node_map(node_map: &IndexMap<Felt, MerkleNode>) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyMerkleNodeMap for Chain
where
    Chain: CanRaiseError<String>,
{
    fn verify_merkle_node_map(node_map: &IndexMap<Felt, MerkleNode>) -> Result<(), Self::Error> {
        for (hash, node) in node_map.iter() {
            Chain::verify_merkle_node(hash, node)?;
        }

        Ok(())
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
