use cgp_core::prelude::*;
use starknet::core::types::Felt;

use crate::impls::encode_mut::felt::EncodeFelt;
use crate::strategy::ViaCairo;

define_components! {
    CairoEncodeMutComponents {
        (ViaCairo, Felt): EncodeFelt,
    }
}
