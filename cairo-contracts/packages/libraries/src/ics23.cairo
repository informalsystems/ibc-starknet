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

#[starknet::contract]
pub mod IIcs23Lib {
    use ics23::{verify_membership, verify_non_membership};
    use super::*;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl IIcs23Impl of super::IIcs23<ContractState> {
        fn verify_membership(
            self: @ContractState,
            specs: Array<ProofSpec>,
            proofs: Array<Proof>,
            root: RootBytes,
            keys: Array<KeyBytes>,
            value: ValueBytes,
            index: u32,
        ) {
            verify_membership(specs, @proofs, root, keys, value, index)
        }

        fn verify_non_membership(
            self: @ContractState,
            specs: Array<ProofSpec>,
            proofs: Array<Proof>,
            root: RootBytes,
            keys: Array<KeyBytes>,
        ) {
            verify_non_membership(specs, @proofs, root, keys)
        }
    }
}
