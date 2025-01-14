use core::poseidon::poseidon_hash_span;

pub trait ValidateBasic<T> {
    fn validate_basic(self: @T);
}

pub trait ComputeKey<T, +Serde<T>, +Drop<T>> {
    fn key(self: @T) -> felt252 {
        poseidon_hash(self)
    }
}

pub fn poseidon_hash<T, +Serde<T>, +Drop<T>>(data: @T) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(data);
    key_builder.key()
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

    fn key(self: @LocalKeyBuilder) -> felt252 {
        poseidon_hash_span(self.data.span())
    }
}

#[derive(Drop, Default, Serde)]
pub struct RemotePathBuilder {
    pub path: ByteArray,
}

#[generate_trait]
pub impl RemotePathBuilderImpl of RemotePathBuilderTrait {
    fn init<T, +Into<T, ByteArray>, +Drop<T>>(base: T) -> RemotePathBuilder {
        RemotePathBuilder { path: base.into() }
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

#[cfg(test)]
mod tests {
    use core::hash::HashStateTrait;
    use core::poseidon::{PoseidonTrait, poseidon_hash_span};
    use super::LocalKeyBuilderImpl;

    #[test]
    fn test_poseidon_hash() {
        // https://github.com/starkware-libs/cairo/blob/dff35c09bfaa1ae0969c48ce4e103bad46d5fe50/corelib/src/poseidon.cairo#L128

        let data = array![1, 2];
        let hash = poseidon_hash_span(data.span());
        assert_eq!(hash, 0x0371cb6995ea5e7effcd2e174de264b5b407027a75a231a70c2c8d196107f0e7);
    }

    #[test]
    fn test_poseidon_update() {
        // https://github.com/starkware-libs/cairo/blob/dff35c09bfaa1ae0969c48ce4e103bad46d5fe50/corelib/src/poseidon.cairo#L99

        let mut state = PoseidonTrait::new();
        state = state.update(1);
        state = state.update(2);
        let hash = state.finalize();
        assert_eq!(hash, 0x0371cb6995ea5e7effcd2e174de264b5b407027a75a231a70c2c8d196107f0e7);
    }

    #[derive(Drop, Serde)]
    pub struct Foo {
        pub foo: Array<ByteArray>,
    }

    fn direct_key(data: @Foo) -> felt252 {
        let mut key_builder = LocalKeyBuilderImpl::init();
        key_builder.append_serde(data);
        key_builder.key()
    }

    fn manual_key(data: @Foo) -> felt252 {
        let mut key_builder = LocalKeyBuilderImpl::init();
        let mut data_span = data.foo.span();
        while let Option::Some(value) = data_span.pop_front() {
            key_builder.append_serde(value);
        };
        key_builder.key()
    }

    #[test]
    fn test_struct_key() {
        // depending on how you serialize the struct, the key will be different
        let value = Foo { foo: array!["hello", "world"] };
        assert_eq!(
            direct_key(@value), 0x8a7f3b665d11363ad83b7fb1ee769c513497f2353a70250b0bddcaaaf6458b
        );
        assert_eq!(
            manual_key(@value), 0x1241da65bd34c17206635225f7a86ab41089a0e14ced6dc24b3b831c78ff4b4
        );
    }
}
