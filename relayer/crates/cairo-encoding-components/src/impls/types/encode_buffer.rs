use cgp_core::Async;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::traits::encode_mut::ProvideEncodeBufferType;

pub struct ProvideVecEncodeBuffer;

impl<Encoding: Async> ProvideEncodeBufferType<Encoding> for ProvideVecEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>,
{
    type EncodeBuffer = Vec<Felt>;

    fn to_encoded(buffer: Vec<Felt>) -> Vec<Felt> {
        buffer
    }
}
