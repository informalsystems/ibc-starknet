use cgp::prelude::*;

#[cgp_type]
pub trait HasCommitmentPathType {
    type CommitmentPath;
}

#[cgp_type]
pub trait HasCommitmentValueType {
    type CommitmentValue;
}
