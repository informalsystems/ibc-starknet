use alloc::vec::Vec;

use indexmap::IndexMap;
use starknet_core::types::{ContractsProof, Felt, MerkleNode, StorageProof};
use starknet_crypto_lib::StarknetCryptoFunctions;

use crate::StorageError;

pub fn validate_storage_proof<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    proof: &StorageProof,
) -> Result<(), StorageError> {
    validate_merkle_node_map(
        crypto_lib,
        &proof.classes_proof,
        &[proof.global_roots.classes_tree_root],
    )?;

    validate_merkle_node_map(
        crypto_lib,
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
        validate_merkle_node_map(crypto_lib, storage_entry, &contract_roots)?;
    }

    validate_contracts_proof(crypto_lib, &proof.contracts_proof)?;

    Ok(())
}

fn validate_merkle_node_map<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    node_map: &IndexMap<Felt, MerkleNode>,
    roots: &[Felt],
) -> Result<(), StorageError> {
    for (hash, node) in node_map.iter() {
        validate_merkle_node(crypto_lib, hash, node)?;
    }

    for (hash, node) in node_map.iter() {
        validate_merkle_node_parent(hash, node_map, roots)?;
    }

    Ok(())
}

fn validate_merkle_node_parent(
    hash: &Felt,
    node_map: &IndexMap<Felt, MerkleNode>,
    roots: &[Felt],
) -> Result<(), StorageError> {
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

    Err(StorageError::MissingParentNode)
}

fn validate_merkle_node<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    node_hash: &Felt,
    node: &MerkleNode,
) -> Result<(), StorageError> {
    match node {
        MerkleNode::BinaryNode(node) => {
            let expected = crypto_lib.pedersen_hash(&node.left, &node.right);

            if &expected != node_hash {
                return Err(StorageError::MismatchBinaryHash);
            }
        }

        MerkleNode::EdgeNode(node) => {
            let expected = crypto_lib.pedersen_hash(&node.child, &node.path) + node.length;

            if &expected != node_hash {
                return Err(StorageError::MismatchEdgeHash);
            }
        }
    }

    Ok(())
}

fn validate_contracts_proof<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    contracts_proof: &ContractsProof,
) -> Result<(), StorageError> {
    for contract_leaf in contracts_proof.contract_leaves_data.iter() {
        let storage_root = contract_leaf
            .storage_root
            .ok_or(StorageError::MissingStorageRoot)?;

        let contract_hash = crypto_lib.pedersen_hash(
            &crypto_lib.pedersen_hash(
                &crypto_lib.pedersen_hash(&contract_leaf.class_hash, &storage_root),
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
            .ok_or(StorageError::MissingContractHash)?;
    }

    Ok(())
}
