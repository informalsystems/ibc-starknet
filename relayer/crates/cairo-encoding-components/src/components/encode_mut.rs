#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use starknet::core::types::{Felt, U256};

    use crate::impls::encode_mut::array::EncodeArray;
    use crate::impls::encode_mut::bool::EncodeBool;
    use crate::impls::encode_mut::felt::EncodeFelt;
    use crate::impls::encode_mut::from_u128::EncodeFromU128;
    use crate::impls::encode_mut::string::EncodeUtf8String;
    use crate::impls::encode_mut::u128::EncodeU128;
    use crate::impls::encode_mut::u256::EncodeU256;
    use crate::impls::encode_mut::unit::EncodeNothing;
    use crate::impls::encode_mut::vec::EncodeList;
    use crate::strategy::ViaCairo;

    cgp_preset! {
        CairoEncodeMutComponents {
            (ViaCairo, Felt): EncodeFelt,
            (ViaCairo, u128): EncodeU128,
            (ViaCairo, U256): EncodeU256,
            (ViaCairo, Vec<u8>): EncodeList,
            (ViaCairo, Vec<u32>): EncodeList,
            (ViaCairo, Vec<Felt>): EncodeList,
            (ViaCairo, bool): EncodeBool,
            (ViaCairo, u8): EncodeFromU128,
            (ViaCairo, u64): EncodeFromU128,
            (ViaCairo, u32): EncodeFromU128,
            (ViaCairo, usize): EncodeFromU128,
            (ViaCairo, String): EncodeUtf8String,
            (ViaCairo, ()): EncodeNothing,
            (ViaCairo, Nil): EncodeNothing,
            (ViaCairo, Vec<String>): EncodeList,
            (ViaCairo, [String; 2]): EncodeArray,
            (ViaCairo, [u32; 8]): EncodeArray,
        }
    }
}
