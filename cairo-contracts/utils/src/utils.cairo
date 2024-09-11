use core::hash::{HashStateTrait, HashStateExTrait};
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;

pub trait ValidateBasicTrait<T> {
    fn validate_basic(self: @T);
}

pub trait ComputeKeyTrait<T, +Serde<T>, +Drop<T>> {
    fn compute_key(self: @T) -> felt252 {
        poseidon_hash(self)
    }
}

pub fn poseidon_hash<T, +Serde<T>, +Drop<T>>(data: @T) -> felt252 {
    let mut key_builder = KeyBuilderImpl::init();
    key_builder.append_serde(data);
    key_builder.compute_key()
}

#[derive(Drop, Serde)]
pub struct KeyBuilder {
    pub data: Array<felt252>,
}

#[generate_trait]
pub impl KeyBuilderImpl of KeyBuilderTrait {
    fn init() -> KeyBuilder {
        KeyBuilder { data: ArrayTrait::new() }
    }

    fn append_serde<T, +Serde<T>, +Drop<T>>(ref self: KeyBuilder, value: @T) {
        value.serialize(ref self.data);
    }

    fn compute_key(self: @KeyBuilder) -> felt252 {
        PoseidonTrait::new().update(poseidon_hash_span(self.data.span())).finalize()
    }
}
