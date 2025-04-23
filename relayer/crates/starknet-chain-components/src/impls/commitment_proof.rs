use core::num::TryFromIntError;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use indexmap::IndexMap;
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::{Felt, MerkleNode, StorageProof};

use crate::impls::storage_proof::CanValidateStorageProof;
use crate::impls::types::address::StarknetAddress;
use crate::traits::commitment_proof::{
    CanVerifyStarknetMerkleProof, StarknetMerkleProofVerifier,
    StarknetMerkleProofVerifierComponent, StarknetStorageProofVerifier,
    StarknetStorageProofVerifierComponent,
};
use crate::traits::types::commitment::HasMerkleProofType;
use crate::traits::types::storage_proof::HasStorageProofType;

#[cgp_new_provider(StarknetMerkleProofVerifierComponent)]
impl<Chain> StarknetMerkleProofVerifier<Chain> for VerifyStarknetMerkleProof
where
    Chain: HasMerkleProofType<MerkleProof = IndexMap<Felt, MerkleNode>>
        + CanRaiseError<String>
        + CanRaiseError<TryFromIntError>,
{
    fn verify_starknet_merkle_proof(
        nodes: &IndexMap<Felt, MerkleNode>,
        root: Felt,
        path: Felt,
        value: Felt,
    ) -> Result<(), Chain::Error> {
        if path >= Felt::ELEMENT_UPPER_BOUND {
            return Err(Chain::raise_error(format!(
                "commitment path exceeds felt upper bound: {path}"
            )));
        }

        let mut remaining_length: u8 = 251;
        let mut path_bits = &path.to_bits_be()[5..];

        let mut current_node = nodes.get(&root).ok_or_else(|| {
            Chain::raise_error(format!(
                "failed to find root proof node: {}",
                root.to_hex_string()
            ))
        })?;

        while !path_bits.is_empty() {
            match current_node {
                MerkleNode::BinaryNode(node) => {
                    let next_bit = path_bits[0];

                    let next_root = if next_bit { node.right } else { node.left };

                    current_node = nodes.get(&next_root).ok_or_else(|| {
                        Chain::raise_error(format!(
                            "failed to find proof node at: {}",
                            next_root.to_hex_string()
                        ))
                    })?;

                    remaining_length -= 1;
                    path_bits = &path_bits[1..];
                }
                MerkleNode::EdgeNode(node) => {
                    let node_length = u8::try_from(node.length).map_err(Chain::raise_error)?;
                    let node_path_bits = node.path.to_bits_be();

                    if node_length > remaining_length {
                        return Err(Chain::raise_error(format!("invalid edge node with node length {node_length} exceeding remaining length {remaining_length}")));
                    }

                    let skip_length = 251u8 - node_length;
                    for i in 0..(skip_length + 5) {
                        if node_path_bits[usize::from(i)] {
                            return Err(Chain::raise_error(format!(
                                "expect node path bit at index {i} to be zero: {}",
                                node.path
                            )));
                        }
                    }

                    let node_path_slice = &node_path_bits[(skip_length + 5).into()..];
                    let path_bits_slice = &path_bits[0..node_length.into()];

                    if node_path_slice == path_bits_slice {
                        if node_length == remaining_length {
                            if value != Felt::ZERO && value == node.child {
                                return Ok(());
                            } else {
                                return Err(Chain::raise_error(format!(
                                    "child node at path {} contains value {}, but expected {:?}",
                                    node.path.to_hex_string(),
                                    node.child.to_hex_string(),
                                    value
                                )));
                            }
                        } else {
                            current_node = nodes.get(&node.child).ok_or_else(|| {
                                Chain::raise_error(format!(
                                    "failed to find proof node at: {}",
                                    &node.child.to_hex_string()
                                ))
                            })?;

                            remaining_length -= node_length;
                            path_bits = &path_bits[node_length.into()..];
                        }
                    } else if value == Felt::ZERO {
                        return Ok(());
                    } else {
                        return Err(Chain::raise_error(format!("expect value to be present, but non-membership proof is found at {node:?}")));
                    }
                }
            }
        }

        Err(Chain::raise_error(
            "malform proof that exceed maximum depth of 251".to_string(),
        ))
    }
}

#[cgp_new_provider(StarknetStorageProofVerifierComponent)]
impl<Chain> StarknetStorageProofVerifier<Chain> for VerifyStarknetStorageProof
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasStorageProofType<StorageProof = StorageProof>
        + HasMerkleProofType<MerkleProof = IndexMap<Felt, MerkleNode>>
        + CanValidateStorageProof
        + CanVerifyStarknetMerkleProof
        + CanRaiseError<String>
        + CanRaiseError<TryFromIntError>,
{
    fn verify_starknet_storage_proof(
        storage_proof: &StorageProof,
        contract_address: &StarknetAddress,
        path: Felt,
        value: Felt,
    ) -> Result<(), Chain::Error> {
        Chain::validate_storage_proof(storage_proof)?;

        let contract_leaf = storage_proof
            .contracts_proof
            .contract_leaves_data
            .first()
            .ok_or_else(|| Chain::raise_error("contract leaf node not found".to_string()))?;

        let contract_root = contract_leaf
            .storage_root
            .ok_or_else(|| Chain::raise_error("contract storage root not found".to_string()))?;

        let contract_storage_proof = storage_proof
            .contracts_storage_proofs
            .first()
            .ok_or_else(|| Chain::raise_error("contract storage proof not found".to_string()))?;

        let contract_hash = pedersen_hash(
            &pedersen_hash(
                &pedersen_hash(&contract_leaf.class_hash, &contract_root),
                &contract_leaf.nonce,
            ),
            &Felt::ZERO,
        );

        Chain::verify_starknet_merkle_proof(
            &storage_proof.contracts_proof.nodes,
            storage_proof.global_roots.contracts_tree_root,
            contract_address.0,
            contract_hash,
        )?;

        Chain::verify_starknet_merkle_proof(contract_storage_proof, contract_root, path, value)?;

        Ok(())
    }
}
