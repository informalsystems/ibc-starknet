use core::num::TryFromIntError;
/*use core::fmt::Write;
use alloc::boxed::Box;
use alloc::string::String;

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
}*/

#[derive(Debug)]
pub enum StorageError {
    ChildNodeWithZeroValue,

    ChildNodeMismatchValue,

    CommitmentPathExceedUpper,

    Generic(alloc::string::String),

    InvalidEdgeNode,

    InvalidProof,

    MismatchBinaryHash,

    MismatchEdgeHash,

    MismatchPathSize,

    MissingContractLeafNode,

    MissingContractStorageProof,

    MissingContractStorageRoot,

    MissingStorageRoot,

    MissingContractHash,

    MissingParentNode,

    MissingRootProofNode,

    MissingProofNode,

    MissingValue,

    NonZeroBit,

    ZeroEdgeNode,

    TryFromIntError,
}

impl From<TryFromIntError> for StorageError {
    fn from(_e: TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}

impl core::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}

impl core::fmt::Display for StorageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ChildNodeWithZeroValue => write!(f, "Child node contains zero value, which is invalid as the whole node must be omitted"),
            Self::ChildNodeMismatchValue => write!(f, "Child node contains a value that does not match the expected value"),
            Self::CommitmentPathExceedUpper => write!(f, "Commitment path exceeds felt upper bound"),
            Self::Generic(msg) => write!(f, "{msg}"),
            Self::InvalidEdgeNode => write!(f, "Invalid edge node"),
            Self::InvalidProof => write!(f, "Malformed proof that exceeds maximum depth of 251"),
            Self::MismatchBinaryHash => write!(f, "Error validating binary node. Expected hash does not match the computed hash"),
            Self::MismatchEdgeHash => write!(f, "Error validating edge node. Expected hash does not match the computed hash"),
            Self::MismatchPathSize => write!(f, "Sliced paths should have the same size"),
            Self::MissingContractLeafNode => write!(f, "Contract leaf node not found"),
            Self::MissingContractStorageProof => write!(f, "Contract storage proof not found"),
            Self::MissingContractStorageRoot => write!(f, "Contract storage root not found"),
            Self::MissingStorageRoot => write!(f, "Storage root not found"),
            Self::MissingContractHash => write!(f, "Contract hash not found"),
            Self::MissingParentNode => write!(f, "Failed to find parent node for child node"),
            Self::MissingRootProofNode => write!(f, "Failed to find root proof node"),
            Self::MissingProofNode => write!(f, "Failed to find proof node"),
            Self::MissingValue => write!(f, "Expected value to be present, but non-membership proof is found"),
            Self::NonZeroBit => write!(f, "Node path bit at index should be zero"),
            Self::ZeroEdgeNode => write!(f, "Invalid edge node with zero node length"),
            Self::TryFromIntError => write!(f, "Failed to convert integer type"),
        }
    }
}
