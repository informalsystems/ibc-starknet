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
