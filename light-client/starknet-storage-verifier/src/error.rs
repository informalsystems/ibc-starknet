use core::num::TryFromIntError;

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
