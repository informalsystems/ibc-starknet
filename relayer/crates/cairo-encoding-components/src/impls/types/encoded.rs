use cgp_core::Async;
use hermes_encoding_components::traits::encoded::ProvideEncodedType;
use starknet::core::types::Felt;

pub struct ProvideVecFeltEncodedType;

impl<Encoding: Async> ProvideEncodedType<Encoding> for ProvideVecFeltEncodedType {
    type Encoded = Vec<Felt>;
}
