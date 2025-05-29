use cometbft::types::{Options, TrustedBlockState, UntrustedBlockState};
use protobuf::types::wkt::Timestamp;


#[starknet::interface]
pub trait ICometBft<TContractState> {
    fn verify_update_header(
        self: @TContractState,
        untrusted: UntrustedBlockState,
        trusted: TrustedBlockState,
        options: Options,
        now: Timestamp,
    );

    fn verify_misbehaviour_header(
        self: @TContractState,
        untrusted: UntrustedBlockState,
        trusted: TrustedBlockState,
        options: Options,
        now: Timestamp,
    );
}

#[starknet::contract]
pub mod ICometBftLib {
    use cometbft::verifier::{verify_misbehaviour_header, verify_update_header};
    use super::*;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl ICometBftImpl of super::ICometBft<ContractState> {
        fn verify_update_header(
            self: @ContractState,
            untrusted: UntrustedBlockState,
            trusted: TrustedBlockState,
            options: Options,
            now: Timestamp,
        ) {
            verify_update_header(untrusted, trusted, options, now)
        }

        fn verify_misbehaviour_header(
            self: @ContractState,
            untrusted: UntrustedBlockState,
            trusted: TrustedBlockState,
            options: Options,
            now: Timestamp,
        ) {
            verify_misbehaviour_header(untrusted, trusted, options, now)
        }
    }
}
