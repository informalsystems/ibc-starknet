use core::fmt::Debug;

use cgp::prelude::*;
use hermes_encoding_components::traits::{HasDecodeBufferType, MutDecoder, MutDecoderComponent};

pub struct DecodeEnd;

#[derive(Copy, Clone)]
pub struct NonEmptyBuffer;

#[allow(unused_mut)]
#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ()> for DecodeEnd
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<NonEmptyBuffer>,
    for<'a> Encoding::DecodeBuffer<'a>: Iterator,
{
    fn decode_mut(
        _encoding: &Encoding,
        mut buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<(), Encoding::Error> {
        match buffer.next() {
            Some(_) => Err(Encoding::raise_error(NonEmptyBuffer)),
            None => Ok(()),
        }
    }
}

impl Debug for NonEmptyBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expected buffer to be empty at the end of decoding")
    }
}
