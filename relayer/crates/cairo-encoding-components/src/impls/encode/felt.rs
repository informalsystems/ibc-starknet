use core::iter;

use cgp_core::error::{CanRaiseError, HasErrorType};
use starknet::core::types::Felt;

use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};

pub struct EncodeFelt;

#[derive(Debug, Copy, Clone)]
pub struct UnexpectedEndOfBuffer;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Encoding::EncodeBuffer: Extend<Felt>,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Felt,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        buffer.extend(iter::once(*value));

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: HasDecodeBufferType + CanRaiseError<UnexpectedEndOfBuffer>,
    Encoding::DecodeBuffer: Iterator<Item = Felt>,
{
    fn decode_mut(
        _encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer,
    ) -> Result<Felt, Encoding::Error> {
        let value = buffer
            .next()
            .ok_or_else(|| Encoding::raise_error(UnexpectedEndOfBuffer))?;

        Ok(value)
    }
}