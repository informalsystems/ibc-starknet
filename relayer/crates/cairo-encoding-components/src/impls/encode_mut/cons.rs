use core::marker::PhantomData;

use cgp::core::error::HasAsyncErrorType;
use cgp::prelude::Cons;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

pub struct EncoderCons<EncoderA, EncoderB>(pub PhantomData<(EncoderA, EncoderB)>);

impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutEncoder<Encoding, Strategy, Cons<ValueA, ValueB>> for EncoderCons<EncoderA, EncoderB>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    EncoderA: MutEncoder<Encoding, Strategy, ValueA>,
    EncoderB: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        Cons(a, b): &Cons<ValueA, ValueB>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncoderA::encode_mut(encoding, a, buffer)?;
        EncoderB::encode_mut(encoding, b, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, EncoderA, EncoderB, ValueA, ValueB>
    MutDecoder<Encoding, Strategy, Cons<ValueA, ValueB>> for EncoderCons<EncoderA, EncoderB>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    EncoderA: MutDecoder<Encoding, Strategy, ValueA>,
    EncoderB: MutDecoder<Encoding, Strategy, ValueB>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Cons<ValueA, ValueB>, Encoding::Error> {
        let a = EncoderA::decode_mut(encoding, buffer)?;
        let b = EncoderB::decode_mut(encoding, buffer)?;

        Ok(Cons(a, b))
    }
}
