use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{MutDecoder, MutDecoderComponent};
use hermes_encoding_components::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeNothing;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ()> for EncodeNothing
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &(),
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ()> for EncodeNothing
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Nil> for EncodeNothing
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &Nil,
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Nil> for EncodeNothing
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Nil, Encoding::Error> {
        Ok(Nil)
    }
}
