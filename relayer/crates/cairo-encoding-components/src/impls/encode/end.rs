use core::fmt::Debug;

use cgp_core::error::{CanRaiseError, HasErrorType};

use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};

pub struct EncodeEnd;

#[derive(Copy, Clone)]
pub struct NonEmptyBuffer;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ()> for EncodeEnd
where
    Encoding: HasEncodeBufferType + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &(),
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ()> for EncodeEnd
where
    Encoding: HasDecodeBufferType + CanRaiseError<NonEmptyBuffer>,
    for<'a> Encoding::DecodeBuffer<'a>: Iterator,
{
    fn decode_mut(
        _encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
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
