use std::num::TryFromIntError;

use starknet_core::types::{BinaryNode, ContractLeafData, EdgeNode};
use starknet_crypto::Felt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("child node at path {0} contains 0 value, which is invalid as the whole node must be omitted")]
    ChildNodeWithZeroValue(String),
    #[error("child node at path {0} contains value {1}, but expected {2:?}")]
    ChildNodeMismatchValue(String, String, Felt),
    #[error("commitment path exceeds felt upper bound: {0}")]
    CommitmentPathExceedUpper(Felt),
    #[error("generic storage error: {0}")]
    Generic(String),
    #[error("invalid edge node with node length {0} exceeding remaining length {1}")]
    InvalidEdgeNode(u8, u8),
    #[error("malform proof that exceed maximum depth of 251")]
    InvalidProof,
    #[error("error validating binary node {0:?}. expected hash: {1}, got: {2}")]
    MismatchBinaryHash(Box<BinaryNode>, Felt, Felt),
    #[error("error validating edge node {0:?}. expected hash: {1}, got: {2}")]
    MismatchEdgeHash(Box<EdgeNode>, Felt, Felt),
    #[error("sliced paths should have the same size")]
    MismatchPathSize,
    #[error("contract leaf node not found")]
    MissingContractLeafNode,
    #[error("contract storage proof not found")]
    MissingContractStorageProof,
    #[error("contract storage root not found")]
    MissingContractStorageRoot,
    #[error("storage root not found at {0:?}")]
    MissingStorageRoot(ContractLeafData),
    #[error("contract hash {0} for {1:?} not found in contract proof nodes")]
    MissingContractHash(String, Box<ContractLeafData>),
    #[error("failed to find parent node for child node with hash {0}")]
    MissingParentNode(String),
    #[error("failed to find root proof node: {0}")]
    MissingRootProofNode(String),
    #[error("failed to find proof node at: {0}")]
    MissingProofNode(String),
    #[error("expect value to be present, but non-membership proof is found at {0:?}")]
    MissingValue(EdgeNode),
    #[error("expect node path bit at index {0} to be zero: {1}")]
    NonZeroBit(u8, Felt),
    #[error("invalid edge node with 0 node length")]
    ZeroEdgeNode,
}

impl From<TryFromIntError> for StorageError {
    fn from(e: TryFromIntError) -> Self {
        Self::Generic(format!("generic storage error: {e}"))
    }
}
