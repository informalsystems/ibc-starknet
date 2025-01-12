// https://github.com/starkware-industries/poseidon/blob/main/poseidon3.txt

// Constants for the Poseidon-3 hash function.
pub const RATE: usize = 2;
pub const CAPACITY: usize = 1;
pub const FULL_ROUNDS: usize = 8;
pub const PARTIAL_ROUNDS: usize = 83;

pub const RATE_PLUS_1: usize = RATE + 1;

pub const MDS: [[i64; RATE_PLUS_1]; RATE_PLUS_1] = [[3, 1, 1], [1, -1, 1], [1, 1, -2]];

// we don't need to maintain ROUND_KEYS, as ROUND_KEYS[id] = Felt(sha256("Hades{id}"))
// ref: https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/cairo/common/poseidon_utils.py#L15
//
// pub const ROUND_KEYS: [[&str; 3]; N_ROUNDS] = ...
