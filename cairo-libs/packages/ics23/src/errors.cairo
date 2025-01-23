pub mod ICS23Errors {
    pub const MISSING_MERKLE_PROOF: felt252 = 'ICS23: missing merkle proof';
    pub const MISSING_KEY: felt252 = 'ICS23: missing key';
    pub const MISSING_VALUE: felt252 = 'ICS23: missing value';
    pub const MISSING_CHILD_HASH: felt252 = 'ICS23: missing child hash';
    pub const MISMATCHED_KEY: felt252 = 'ICS23: mismatched key';
    pub const MISMATCHED_VALUE: felt252 = 'ICS23: mismatched value';
    pub const MISMATCHED_ROOT: felt252 = 'ICS23: mismatched root';
    pub const MISMATCHED_NUM_OF_PROOFS: felt252 = 'ICS23: mismatched num of proofs';
    pub const INVALID_MERKLE_PROOF: felt252 = 'ICS23: invalid merkle proof';
    pub const INVALID_PROOF_TYPE: felt252 = 'ICS23: invalid proof type';
    pub const INVALID_INNER_SPEC: felt252 = 'ICS23: invalid inner spec';
    pub const INVALID_DEPTH_RANGE: felt252 = 'ICS23: invalid depth range';
    pub const UNSUPPORTED_HASH_OP: felt252 = 'ICS23: unsupported hash op';
    pub const ZERO_MERKLE_ROOT: felt252 = 'ICS23: zero merkle root';
}
