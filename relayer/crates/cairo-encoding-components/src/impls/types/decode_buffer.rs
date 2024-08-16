use core::iter::Peekable;
use core::ops::Deref;
use core::slice::Iter;

use hermes_encoding_components::traits::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::traits::decode_mut::{DecodeBufferPeeker, HasDecodeBufferType, ProvideDecodeBufferType};

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
    fn peek<'a>(buffer: &'a mut Peekable<Iter<'_, Felt>>) -> Option<&'a Felt> {
        buffer.peek().map(Deref::deref)
    }
}
