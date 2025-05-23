use core::iter::Peekable;
use core::ops::Deref;
use core::slice::Iter;

use hermes_encoding_components::traits::{
    DecodeBufferBuilder, DecodeBufferBuilderComponent, DecodeBufferPeeker,
    DecodeBufferPeekerComponent, DecodeBufferTypeComponent, HasDecodeBufferType, HasEncodedType,
    ProvideDecodeBufferType,
};
use hermes_prelude::*;
use starknet::core::types::Felt;

pub struct ProvideVecIterDecodeBuffer;

#[cgp_provider(DecodeBufferTypeComponent)]
impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideVecIterDecodeBuffer {
    type DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>;
}

#[cgp_provider(DecodeBufferBuilderComponent)]
impl<Encoding> DecodeBufferBuilder<Encoding> for ProvideVecIterDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>>,
{
    fn from_encoded<'a>(encoded: &'a Vec<Felt>) -> Peekable<Iter<'a, Felt>> {
        encoded.iter().peekable()
    }
}

#[cgp_provider(DecodeBufferPeekerComponent)]
impl<Encoding> DecodeBufferPeeker<Encoding, Felt> for ProvideVecIterDecodeBuffer
where
    Encoding: for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>>,
{
    fn peek_decode_buffer<'a>(buffer: &'a mut Peekable<Iter<'_, Felt>>) -> Option<&'a Felt> {
        buffer.peek().map(Deref::deref)
    }
}
