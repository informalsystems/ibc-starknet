use core::hash::{HashStateTrait, HashStateExTrait};
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;

pub trait ValidateBasicTrait<T> {
    fn validate_basic(self: @T);
}

pub trait ComputeKeyTrait<T> {
    fn compute_key(self: @T) -> felt252;
}

pub fn poseidon_hash<T, +Serde<T>>(data: @T) -> felt252 {
    let mut serialized_port_id: Array<felt252> = ArrayTrait::new();
    Serde::serialize(data, ref serialized_port_id);
    PoseidonTrait::new().update(poseidon_hash_span(serialized_port_id.span())).finalize()
}
