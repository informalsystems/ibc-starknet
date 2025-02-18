use cgp::prelude::*;
use hermes_encoding_components::traits::types::encoded::{
    EncodedTypeComponent, ProvideEncodedType,
};
use starknet::core::types::Felt;

pub struct ProvideVecFeltEncodedType;

#[cgp_provider(EncodedTypeComponent)]
impl<Encoding: Async> ProvideEncodedType<Encoding> for ProvideVecFeltEncodedType {
    type Encoded = Vec<Felt>;
}
