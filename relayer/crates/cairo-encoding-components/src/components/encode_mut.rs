use cgp_core::prelude::*;
use starknet::core::types::Felt;

use crate::impls::encode_mut::felt::EncodeFelt;

define_components! {
    CairoEncodeMutComponents {
        Felt: EncodeFelt,
    }
}
