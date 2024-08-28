use core::iter::Peekable;
use core::ops::Deref;
use core::slice::Iter;

use hermes_encoding_components::traits::decode_mut::DecodeBufferPeeker;
use hermes_encoding_components::traits::types::decode_buffer::{
    HasDecodeBufferType, ProvideDecodeBufferType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

pub struct ProvideVecIterDecodeBuffer;

impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideVecIterDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>,
{
    type DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>;

    fn from_encoded<'a>(encoded: &'a Vec<Felt>) -> Peekable<Iter<'a, Felt>> {
        encoded.iter().peekable()
    }
}

impl<Encoding> DecodeBufferPeeker<Encoding, Felt> for ProvideVecIterDecodeBuffer
where
    Encoding: for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>>,
{
    fn peek_decode_buffer<'a>(buffer: &'a mut Peekable<Iter<'_, Felt>>) -> Option<&'a Felt> {
        buffer.peek().map(Deref::deref)
    }
}
