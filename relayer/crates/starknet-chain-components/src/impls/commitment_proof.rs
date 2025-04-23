use core::num::TryFromIntError;

use cgp::prelude::*;
use starknet::core::types::{Felt, MerkleNode};

use crate::traits::commitment_proof::{MerkleProofVerifier, MerkleProofVerifierComponent};
use crate::traits::types::commitment::{
    HasCommitmentPathType, HasCommitmentValueType, HasMerkleProofType,
};
use crate::types::merkle_proof::StarknetMerkleProof;

#[cgp_new_provider(MerkleProofVerifierComponent)]
impl<Chain> MerkleProofVerifier<Chain> for VerifyStarknetMerkleProof
where
    Chain: HasMerkleProofType<MerkleProof = StarknetMerkleProof>
        + HasCommitmentPathType<CommitmentPath = Felt>
        + HasCommitmentValueType<CommitmentValue = Felt>
        + CanRaiseError<String>
        + CanRaiseError<TryFromIntError>,
{
    fn verify_merkle_proof(
        proof: &StarknetMerkleProof,
        path: &Felt,
        value: Option<&Felt>,
    ) -> Result<(), Chain::Error> {
        if path >= &Felt::ELEMENT_UPPER_BOUND {
            return Err(Chain::raise_error(format!(
                "commitment path exceeds felt upper bound: {path}"
            )));
        }

        let mut remaining_length: u8 = 251;
        let mut path_bits = &path.to_bits_be()[5..];

        let nodes = &proof.proof_nodes;
        let mut current_node = nodes.get(&proof.root).ok_or_else(|| {
            Chain::raise_error(format!("failed to find root proof node: {}", proof.root))
        })?;

        while !path_bits.is_empty() {
            match current_node {
                MerkleNode::BinaryNode(node) => {
                    let next_bit = path_bits[0];

                    let next_root = if next_bit { node.left } else { node.right };

                    current_node = nodes.get(&next_root).ok_or_else(|| {
                        Chain::raise_error(format!("failed to find proof node at: {}", next_root))
                    })?;

                    remaining_length -= 1;
                    path_bits = &path_bits[1..];
                }
                MerkleNode::EdgeNode(node) => {
                    let node_length = u8::try_from(node.length).map_err(Chain::raise_error)?;
                    let node_path_bits = node.path.to_bits_be();

                    if node_length > remaining_length.into() {
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

                    let node_path_slice = &node_path_bits[0..skip_length.into()];
                    let path_bits_slice = &path_bits[0..node_length.into()];

                    if node_path_slice == path_bits_slice {
                        if node_length == remaining_length {
                            if value == Some(&node.child) {
                                return Ok(());
                            } else {
                                return Err(Chain::raise_error(format!(
                                    "child node at path {} contains value {}, but expected {:?}",
                                    node.path, node.child, value
                                )));
                            }
                        } else {
                            current_node = nodes.get(&node.child).ok_or_else(|| {
                                Chain::raise_error(format!(
                                    "failed to find proof node at: {}",
                                    &node.child
                                ))
                            })?;

                            remaining_length -= node_length;
                            path_bits = &path_bits[node_length.into()..];
                        }
                    } else {
                        if value.is_none() {
                            return Ok(());
                        } else {
                            return Err(Chain::raise_error(format!("expect value to be present, but non-membership proof is found at {node:?}")));
                        }
                    }
                }
            }
        }

        return Err(Chain::raise_error(format!(
            "malform proof that exceed maximum depth of 251"
        )));
    }
}
