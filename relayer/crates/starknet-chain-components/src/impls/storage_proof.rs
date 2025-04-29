use hermes_prelude::{CanRaiseError, HasErrorType};
use indexmap::IndexMap;
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::{ContractsProof, Felt, MerkleNode, StorageProof};

use crate::traits::types::storage_proof::HasStorageProofType;

/**
    Try to validate the structure of storage proof according to:
    <https://docs.starknet.io/architecture-and-concepts/network-architecture/starknet-state/>.

    Here, we mainly validate that the hash values and parent relationships are valid.
    The actual membership proofs are verified later, depending on the paths and values
    that we want to validate.

    The main advantage of separating the two is that we can validate a storage proof once,
    and then use it to validate multiple Merkle membership proofs.
*/
pub trait CanValidateStorageProof: HasStorageProofType + HasErrorType {
    fn validate_storage_proof(proof: &Self::StorageProof) -> Result<(), Self::Error>;
}

impl<Chain> CanValidateStorageProof for Chain
where
    Chain: HasStorageProofType<StorageProof = StorageProof> + CanRaiseError<String>,
{
    fn validate_storage_proof(proof: &StorageProof) -> Result<(), Self::Error> {
        Chain::validate_merkle_node_map(
            &proof.classes_proof,
            &[proof.global_roots.classes_tree_root],
        )?;

        Chain::validate_merkle_node_map(
            &proof.contracts_proof.nodes,
            &[proof.global_roots.contracts_tree_root],
        )?;

        let contract_roots = proof
            .contracts_proof
            .contract_leaves_data
            .iter()
            .flat_map(|leaf| leaf.storage_root.into_iter())
            .collect::<Vec<_>>();

        for storage_entry in proof.contracts_storage_proofs.iter() {
            Chain::validate_merkle_node_map(storage_entry, &contract_roots)?;
        }

        Chain::validate_contracts_proof(&proof.contracts_proof)?;

        Ok(())
    }
}

pub trait CanValidateContractsProof: HasErrorType {
    fn validate_contracts_proof(contracts_proof: &ContractsProof) -> Result<(), Self::Error>;
}

impl<Chain> CanValidateContractsProof for Chain
where
    Chain: CanRaiseError<String>,
{
    fn validate_contracts_proof(contracts_proof: &ContractsProof) -> Result<(), Self::Error> {
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

            contracts_proof
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
        }

        Ok(())
    }
}

pub trait CanValidateMerkleNodeMap: HasErrorType {
    fn validate_merkle_node_map(
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &[Felt],
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanValidateMerkleNodeMap for Chain
where
    Chain: CanRaiseError<String>,
{
    fn validate_merkle_node_map(
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &[Felt],
    ) -> Result<(), Self::Error> {
        for (hash, node) in node_map.iter() {
            Chain::validate_merkle_node(hash, node)?;
        }

        for (hash, node) in node_map.iter() {
            Chain::validate_merkle_node_parent(hash, node_map, roots)?;
        }

        Ok(())
    }
}

/**
   Validates that each node entry is either the child of another entry,
   or is one of the root nodes.
*/
pub trait CanValidateMerkleNodeParent: HasErrorType {
    fn validate_merkle_node_parent(
        node_hash: &Felt,
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &[Felt],
    ) -> Result<(), Self::Error>;
}

impl<Chain> CanValidateMerkleNodeParent for Chain
where
    Chain: CanRaiseError<String>,
{
    fn validate_merkle_node_parent(
        hash: &Felt,
        node_map: &IndexMap<Felt, MerkleNode>,
        roots: &[Felt],
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
                        // Note: we cannot really know whether a child subtree is trustworthy
                        // with just a naive iteration here. It is possible for one to "embed"
                        // an entire subtree inside a leaf node, and this validation will still
                        // succeed. This may or may not be the intended behavior, but for formatting
                        // purpose, we consider this as a valid tree.
                        //
                        // The actual validation of whether to walk "into" the subtree will depend
                        // on the merkle proof verification with specific Merkle path. There, we will
                        // keep track of the length (depth) and path so that we don't accidentally
                        // go beyond a subtree. We don't do that here, as it would require significant
                        // performance overhead to really keep track of the depth of the tree.

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

pub trait CanValidateMerkleNode: HasErrorType {
    fn validate_merkle_node(node_hash: &Felt, node: &MerkleNode) -> Result<(), Self::Error>;
}

impl<Chain> CanValidateMerkleNode for Chain
where
    Chain: CanRaiseError<String>,
{
    fn validate_merkle_node(node_hash: &Felt, node: &MerkleNode) -> Result<(), Self::Error> {
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
