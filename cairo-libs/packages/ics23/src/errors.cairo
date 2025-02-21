pub mod ICS23Errors {
    pub const MISSING_MERKLE_PROOF: felt252 = 'ICS23: missing merkle proof';
    pub const MISSING_KEY: felt252 = 'ICS23: missing key';
    pub const MISSING_VALUE: felt252 = 'ICS23: missing value';
    pub const MISSING_LEAF_DATA: felt252 = 'ICS23: missing leaf data';
    pub const MISSING_CHILD_HASH: felt252 = 'ICS23: missing child hash';
    pub const MISMATCHED_KEY: felt252 = 'ICS23: mismatched key';
    pub const MISMATCHED_VALUE: felt252 = 'ICS23: mismatched value';
    pub const MISMATCHED_ROOT: felt252 = 'ICS23: mismatched root';
    pub const MISMATCHED_NUM_OF_PROOFS: felt252 = 'ICS23: mismatched num of proofs';
    pub const INVALID_MERKLE_PROOF: felt252 = 'ICS23: invalid merkle proof';
    pub const INVALID_PROOF_TYPE: felt252 = 'ICS23: invalid proof type';
    pub const INVALID_INNER_SPEC: felt252 = 'ICS23: invalid inner spec';
    pub const INVALID_INNER_OP_SIZE: felt252 = 'ICS23: invalid inner op size';
    pub const INVALID_INNER_PREFIX: felt252 = 'ICS23: invalid inner prefix';
    pub const INVALID_INNER_PREFIX_LEN: felt252 = 'ICS23: invalid inner prefix len';
    pub const INVALID_INNER_SUFFIX: felt252 = 'ICS23: invalid inner suffix';
    pub const INVALID_HASH_OP: felt252 = 'ICS23: invalid hash op';
    pub const INVALID_PREHASH_KEY: felt252 = 'ICS23: invalid prehash key';
    pub const INVALID_PREHASH_VALUE: felt252 = 'ICS23: invalid prehash value';
    pub const INVALID_LENGTH_OP: felt252 = 'ICS23: invalid length op';
    pub const INVALID_DEPTH_RANGE: felt252 = 'ICS23: invalid depth range';
    pub const INVALID_LEAF_PREFIX: felt252 = 'ICS23: invalid leaf prefix';
    pub const INVALID_IAVL_HEIGHT_PREFIX: felt252 = 'ICS23: invalid height prefix';
    pub const UNSUPPORTED_HASH_OP: felt252 = 'ICS23: unsupported hash op';
    pub const ZERO_MERKLE_ROOT: felt252 = 'ICS23: zero merkle root';
    pub const ZERO_CHILD_SIZE: felt252 = 'ICS23: zero child size';
}
