use cgp::core::error::HasAsyncErrorType;
use cgp::prelude::Nil;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncodeNothing;

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
