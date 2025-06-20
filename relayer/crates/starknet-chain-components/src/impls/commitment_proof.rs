use core::num::TryFromIntError;

use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_prelude::*;
use indexmap::IndexMap;
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::Felt;
use starknet_v14::core::types::{MerkleNode, StorageProof};

use crate::impls::{CanValidateStorageProof, StarknetAddress};
use crate::traits::{
    CanVerifyStarknetMerkleProof, HasMerkleProofType, HasStorageProofType,
    StarknetMerkleProofVerifier, StarknetMerkleProofVerifierComponent,
    StarknetStorageProofVerifier, StarknetStorageProofVerifierComponent,
};

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
        // The max value of a felt key is 251 bits. But when we convert the Felt type in Rust into
        // bits, it returns 256 bits with the first 5 bits being 0. So we have to trim the bits out
        // before starting the verification.

        // This check also ensures that the first 5 bits is always 0.
        if path >= Felt::ELEMENT_UPPER_BOUND {
            return Err(Chain::raise_error(format!(
                "commitment path exceeds felt upper bound: {path}"
            )));
        }

        let mut remaining_length: u8 = 251;

        // Use to_bits_be, which starts from the most significant bit, i.e. reverse order
        let mut path_bits = &path.to_bits_be()[5..];

        let mut current_node = nodes.get(&root).ok_or_else(|| {
            Chain::raise_error(format!(
                "failed to find root proof node: {}",
                root.to_hex_string()
            ))
        })?;

        // Keep interating until all path bits are consumed.
        // TODO: ensure that the loop terminates
        while !path_bits.is_empty() {
            match current_node {
                MerkleNode::BinaryNode(node) => {
                    // When encountering a binary node, we use the next bit to determine
                    // whether to go left or right.

                    let next_bit = path_bits[0];

                    let next_root = if next_bit { node.right } else { node.left };

                    current_node = nodes.get(&next_root).ok_or_else(|| {
                        Chain::raise_error(format!(
                            "failed to find proof node at: {}",
                            next_root.to_hex_string()
                        ))
                    })?;

                    // Slice out the one bit and continue with hte next iteration.

                    remaining_length -= 1;
                    path_bits = &path_bits[1..];
                }
                MerkleNode::EdgeNode(node) => {
                    // When encountering an edge node, we traverse down multiple depths that contain only one
                    // non-zero branch.

                    // How many bits of path to skip
                    let node_length = u8::try_from(node.length).map_err(Chain::raise_error)?;

                    // We should at most go down 251 depth. So if the length is greater than that, it is malformed.
                    if node_length > remaining_length {
                        return Err(Chain::raise_error(format!("invalid edge node with node length {node_length} exceeding remaining length {remaining_length}")));
                    }

                    // The node length must not be zero, or else we can get stuck in an infinite loop and cannot proceed.
                    if node_length == 0 {
                        return Err(Chain::raise_error(
                            "invalid edge node with 0 node length".to_string(),
                        ));
                    }

                    // The raw path bits that contains leading zeros
                    let node_path_bits = node.path.to_bits_be();

                    // We want to calculate how many zero bits to skip. Since the raw bits are 256 and only
                    // {node_length} bits are filled, the number of zero bits are 255 - node_length.
                    // We just split calculation so that the value can fit inside u8.
                    let skip_length = 251u8 - node_length + 5;

                    // Check that the bits that we skip must all be 0.
                    for i in 0..(skip_length) {
                        if node_path_bits[usize::from(i)] {
                            return Err(Chain::raise_error(format!(
                                "expect node path bit at index {i} to be zero: {}",
                                node.path
                            )));
                        }
                    }

                    // Slice out the bits that we have skipped, keeping only the valid path bits.
                    let node_path_slice = &node_path_bits[skip_length.into()..];

                    // Slice out same length of path bits that we want to verify against.
                    let path_bits_slice = &path_bits[0..node_length.into()];

                    // If the two slices have different size, then we messed up somewhere in our code.
                    if node_path_slice.len() != path_bits_slice.len() {
                        return Err(Chain::raise_error(
                            "internal error: sliced paths should have the same size".to_string(),
                        ));
                    }

                    if node_path_slice == path_bits_slice {
                        // If the path bits matches, then we are in the correct path down

                        if node_length == remaining_length {
                            // If there is no remaining length after this, we have reached the bottom of the tree

                            if node.child == Felt::ZERO {
                                return Err(Chain::raise_error(format!(
                                    "child node at path {} contains 0 value, which is invalid as the whole node must be omitted",
                                    node.path.to_hex_string(),
                                )));
                            }

                            if value == node.child {
                                // Succeed if the leaf node contains the same value as we expected.

                                return Ok(());
                            } else {
                                // Failed if the leaf node contains a different value.

                                return Err(Chain::raise_error(format!(
                                    "child node at path {} contains value {}, but expected {:?}",
                                    node.path.to_hex_string(),
                                    node.child.to_hex_string(),
                                    value
                                )));
                            }
                        } else {
                            // If there are remaining length, this means that there is still a sub-branch
                            // beneath that contains two non-zero nodes.

                            current_node = nodes.get(&node.child).ok_or_else(|| {
                                Chain::raise_error(format!(
                                    "failed to find proof node at: {}",
                                    &node.child.to_hex_string()
                                ))
                            })?;

                            // Slice out the bits that we have traveresed and continue with the next iteration.

                            remaining_length -= node_length;
                            path_bits = &path_bits[node_length.into()..];
                        }
                    } else if value == Felt::ZERO {
                        // If the path don't match, then that implies the value is 0. If the expected value is also 0,
                        // then we get a non-membership proof
                        return Ok(());
                    } else {
                        // Otherwise, the path don't match and the expected value is not zero.
                        // Then we failed to prove that a non-zero value is present in the tree.

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
        // Validate that all hash inside the storage proofs are derived correctly,
        // and all nodes parents converge to the stated roots.
        Chain::validate_storage_proof(storage_proof)?;

        // We assume that the storage proof only contains one contract proof.
        // If there is more than one, it may fail if the contract of interest
        // is not at the first position.

        // TODO: reject if the storage proof contains multiple contract proofs,
        // or use a more strict domain type.

        // Get the state details about the contract, which contains the
        // state root, class hash, and nonce.
        let contract_leaf = storage_proof
            .contracts_proof
            .contract_leaves_data
            .first()
            .ok_or_else(|| Chain::raise_error("contract leaf node not found".to_string()))?;

        // Get the state root of the contract.
        let contract_root = contract_leaf
            .storage_root
            .ok_or_else(|| Chain::raise_error("contract storage root not found".to_string()))?;

        let contract_storage_proof = storage_proof
            .contracts_storage_proofs
            .first()
            .ok_or_else(|| Chain::raise_error("contract storage proof not found".to_string()))?;

        // The contract hash needs to be calculated manually and is not stored in the storage proof.
        let contract_hash = pedersen_hash(
            &pedersen_hash(
                &pedersen_hash(&contract_leaf.class_hash, &contract_root),
                &contract_leaf.nonce,
            ),
            &Felt::ZERO,
        );

        // Verify that the contract root is set at the given global state root, with the
        // contract address being the path.
        Chain::verify_starknet_merkle_proof(
            &storage_proof.contracts_proof.nodes,
            storage_proof.global_roots.contracts_tree_root,
            contract_address.0,
            contract_hash,
        )?;

        // Verify the value within the contract, with the Merkle proof for that contract.
        Chain::verify_starknet_merkle_proof(contract_storage_proof, contract_root, path, value)?;

        Ok(())
    }
}
