use core::slice::Iter;

use hermes_encoding_components::traits::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::traits::decode_mut::ProvideDecodeBufferType;

pub struct ProvideVecIterDecodeBuffer;

impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideVecIterDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>,
{
    type DecodeBuffer<'a> = Iter<'a, Felt>;

    fn from_encoded<'a>(encoded: &'a Vec<Felt>) -> Iter<'a, Felt> {
        encoded.iter()
    }
}
