use cometbft::types::{Options, TrustedBlockState, UntrustedBlockState};
use protobuf::types::wkt::Timestamp;


#[starknet::interface]
pub trait IComet<TContractState> {
    fn verify_update_header(
        self: @TContractState,
        untrusted: UntrustedBlockState,
        trusted: TrustedBlockState,
        options: Options,
        now: Timestamp,
        signature_hints: Array<Array<felt252>>,
    );

    fn verify_misbehaviour_header(
        self: @TContractState,
        untrusted: UntrustedBlockState,
        trusted: TrustedBlockState,
        options: Options,
        now: Timestamp,
        signature_hints: Array<Array<felt252>>,
    );
}

#[starknet::component]
pub mod CometLibComponent {
    use cometbft::verifier::{verify_misbehaviour_header, verify_update_header};
    use super::*;

    #[storage]
    pub struct Storage {}

    #[embeddable_as(CometLib)]
    pub impl CometLibImpl<
        TContractState, +HasComponent<TContractState>,
    > of super::IComet<ComponentState<TContractState>> {
        fn verify_update_header(
            self: @ComponentState<TContractState>,
            untrusted: UntrustedBlockState,
            trusted: TrustedBlockState,
            options: Options,
            now: Timestamp,
            signature_hints: Array<Array<felt252>>,
        ) {
            verify_update_header(untrusted, trusted, options, now, signature_hints)
        }

        fn verify_misbehaviour_header(
            self: @ComponentState<TContractState>,
            untrusted: UntrustedBlockState,
            trusted: TrustedBlockState,
            options: Options,
            now: Timestamp,
            signature_hints: Array<Array<felt252>>,
        ) {
            verify_misbehaviour_header(untrusted, trusted, options, now, signature_hints)
        }
    }
}
