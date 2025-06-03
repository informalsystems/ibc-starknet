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

#[starknet::component]
pub mod Ics23LibComponent {
    use ics23::{verify_membership, verify_non_membership};
    use super::*;

    #[storage]
    pub struct Storage {}

    #[embeddable_as(Ics23Lib)]
    impl Ics23LibImpl<
        TContractState, +HasComponent<TContractState>,
    > of super::IIcs23<ComponentState<TContractState>> {
        fn verify_membership(
            self: @ComponentState<TContractState>,
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
            self: @ComponentState<TContractState>,
            specs: Array<ProofSpec>,
            proofs: Array<Proof>,
            root: RootBytes,
            keys: Array<KeyBytes>,
        ) {
            verify_non_membership(specs, @proofs, root, keys)
        }
    }
}
