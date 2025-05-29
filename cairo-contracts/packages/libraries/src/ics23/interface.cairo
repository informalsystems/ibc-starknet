use ics23::{
    ExistenceProofImpl, KeyBytes, NonExistenceProofImpl, Proof, ProofSpec, RootBytes, ValueBytes,
};

#[starknet::interface]
pub trait IIcs23<TContractState> {
    fn verify_membership(
        self: @TContractState,
        specs: Array<ProofSpec>,
        proofs: Array<Proof>,
        root: RootBytes,
        keys: Array<KeyBytes>,
        value: ValueBytes,
        index: u32,
    );

    fn verify_non_membership(
        self: @TContractState,
        specs: Array<ProofSpec>,
        proofs: Array<Proof>,
        root: RootBytes,
        keys: Array<KeyBytes>,
    );
}
