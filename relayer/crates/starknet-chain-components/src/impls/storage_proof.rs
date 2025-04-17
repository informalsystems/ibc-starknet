use cgp::prelude::{CanRaiseError, HasErrorType};
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::{MerkleNode, StorageProof};

use crate::traits::types::storage_proof::HasStorageProofType;

pub trait CanVerifyStorageProof: HasStorageProofType + HasErrorType {
    fn verify_storage_proof(proof: &Self::StorageProof) -> Result<(), Self::Error>;
}

impl<Chain> CanVerifyStorageProof for Chain
where
    Chain: HasStorageProofType<StorageProof = StorageProof> + CanRaiseError<String>,
{
    fn verify_storage_proof(proof: &StorageProof) -> Result<(), Self::Error> {
        for storage_entry in proof.contracts_storage_proofs.iter() {
            for (hash, node) in storage_entry.iter() {
                match node {
                    MerkleNode::BinaryNode(node) => {
                        let expected = pedersen_hash(&node.left, &node.right);
                        if &expected != hash {
                            return Err(Chain::raise_error(format!(
                                "error validating binary node {node:?}. expected hash: {expected}, got: {hash}"
                            )));
                        }
                    }
                    MerkleNode::EdgeNode(node) => {}
                }
            }
        }

        Ok(())
    }
}
