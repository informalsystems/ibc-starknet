use starknet_core::utils::starknet_keccak;
use starknet_crypto::{pedersen_hash, Felt};

/// Each variant denotes components of a Starknet storage path.
///
/// This representation is not complete, but it works for our IBC
/// contract storage keys.
///
/// #[derive(starknet::Store)]
/// pub struct Bar {
///   buzz: felt252,
///   bull: felt252,
/// }
///
/// #[starknet::storage_node]
/// pub struct Foo {
///   fuzz: felt252,
/// }
///
/// #[storage]
/// pub struct Storage {
///   single: felt252,
///   foo: Foo,
///   val_map: Map<felt252, felt252>,
///   val_vec: Vec<felt252>,
///   bar: Bar,
/// }
///
/// supports:
/// `single`: [Field(b"single")]
/// `foo.fuzz`: [Field(b"foo"), Field(b"fuzz")]
/// `val_map.0x1234`: [Field(b"val_map"), Map(0x1234)]
/// `val_vec.10`: [Field(b"val_vec"), Vec(10)]
///
/// doesn't support custom types:
/// `bar.buzz`: the key is `starknet_storage_key([Field(b"bar")])`
/// `bar.bull` : the key is `starknet_storage_key([Field(b"bar")]) + 1``
///
/// ref: https://book.cairo-lang.org/ch101-01-00-contract-storage.html
pub enum KeyPart<'a> {
    Field(&'a [u8]),
    Map(Felt),
    Vec(u32),
}

impl KeyPart<'_> {
    pub fn hash(&self) -> Felt {
        match self {
            Self::Field(name) => starknet_keccak(name),
            Self::Map(name) => *name,
            Self::Vec(name) => (*name).into(),
        }
    }
}

pub fn starknet_storage_key<const N: usize>(parts: [KeyPart<'_>; N]) -> Felt {
    // left-associative nesting of hashes
    parts
        .map(|part| part.hash())
        .into_iter()
        .reduce(|acc, e| pedersen_hash(&acc, &e))
        .expect("failed to reduce storage key parts")
}
