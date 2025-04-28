use core::iter;

use cgp::prelude::*;
use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use starknet::core::types::Felt;

pub struct EncodeFelt;

#[derive(Debug, Copy, Clone)]
pub struct UnexpectedEndOfBuffer;

#[cgp_provider(MutEncoderComponent)]
#[allow(unused_mut)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    Encoding::EncodeBuffer: Extend<Felt>,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Felt,
        mut buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        buffer.extend(iter::once(*value));

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
#[allow(unused_mut)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<UnexpectedEndOfBuffer>,
    for<'a> Encoding::DecodeBuffer<'a>: CanIterFeltBuffer<'a>,
{
    fn decode_mut(
        _encoding: &Encoding,
        mut buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Felt, Encoding::Error> {
        let value = buffer
            .next()
            .ok_or_else(|| Encoding::raise_error(UnexpectedEndOfBuffer))?;

        Ok(*value)
    }
}

pub trait CanIterFeltBuffer<'a>: Iterator<Item = &'a Felt> {}

impl<'a, Buffer> CanIterFeltBuffer<'a> for Buffer where Buffer: Iterator<Item = &'a Felt> {}
