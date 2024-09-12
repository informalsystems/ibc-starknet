use core::hash::{HashStateTrait, HashStateExTrait};
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;
use starknet_ibc_utils::UtilErrors;

pub trait ValidateBasicTrait<T> {
    fn validate_basic(self: @T);
}

pub trait ComputeKeyTrait<T, +Serde<T>, +Drop<T>> {
    fn compute_key(self: @T) -> felt252 {
        poseidon_hash(self)
    }
}

pub fn poseidon_hash<T, +Serde<T>, +Drop<T>>(data: @T) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(data);
    key_builder.compute_key()
}

#[derive(Drop, Serde)]
pub struct LocalKeyBuilder {
    pub data: Array<felt252>,
}

#[generate_trait]
pub impl LocalKeyBuilderImpl of LocalKeyBuilderTrait {
    fn init() -> LocalKeyBuilder {
        LocalKeyBuilder { data: ArrayTrait::new() }
    }

    fn append_serde<T, +Serde<T>, +Drop<T>>(ref self: LocalKeyBuilder, value: @T) {
        value.serialize(ref self.data);
    }

    fn compute_key(self: @LocalKeyBuilder) -> felt252 {
        PoseidonTrait::new().update(poseidon_hash_span(self.data.span())).finalize()
    }
}

#[derive(Drop, Serde)]
pub struct RemotePathBuilder {
    pub path: ByteArray,
}

#[generate_trait]
pub impl RemotePathBuilderImpl of RemotePathBuilderTrait {
    fn init() -> RemotePathBuilder {
        RemotePathBuilder { path: "" }
    }

    fn append<T, +Into<T, ByteArray>, +Drop<T>>(ref self: RemotePathBuilder, value: T) {
        if self.path.len() > 0 {
            self.path.append(@"/");
        }

        self.path.append(@value.into());
    }

    fn path(self: RemotePathBuilder) -> ByteArray {
        self.path
    }
}
