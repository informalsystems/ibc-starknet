pub mod CometErrors {
    pub const INVALID_COMMIT_HASH: felt252 = 'ICS07: invalid commit hash';
    pub const INVALID_SIGNATURE_COUNT: felt252 = 'ICS07: invalid sig count';
    pub const INVALID_VALIDATOR_SET_HASH: felt252 = 'ICS07: invalid val set hash';
    pub const NON_MONOTONIC_BFT_TIME: felt252 = 'ICS07: non-monotonic bft time';
    pub const TRUSTED_HEADER_EXPIRED: felt252 = 'ICS07: trusted header expired';
    pub const CHAIN_ID_MISMATCH: felt252 = 'ICS07: chain id mismatch';
    pub const NON_MONOTONIC_HEIGHT: felt252 = 'ICS07: non-monotonic height';
    pub const INVALID_NEXT_VALIDATOR_SET: felt252 = 'ICS07: invalid next val set';
    pub const INVALID_SIGNATURE_LENGTH: felt252 = 'ICS07: invalid signature length';
    pub const INVALID_PUBKEY_LENGTH: felt252 = 'ICS07: invalid pubkey length';
    pub const INVALID_ED25519_SIGNATURE: felt252 = 'ICS07: invalid ed25519 sig';
    pub const UNSUPPORTED_PUBKEY_TYPE: felt252 = 'ICS07: unsupported pubkey type';
    pub const INSUFFICIENT_VOTING_POWER: felt252 = 'ICS07: not enough voting power';
    pub const OVERFLOWED_BLOCK_HEIGHT: felt252 = 'ICS07: overflowed block height';
    pub const OVERFLOWED_VOTING_CALC: felt252 = 'ICS07: overflowed voting calc';
}
