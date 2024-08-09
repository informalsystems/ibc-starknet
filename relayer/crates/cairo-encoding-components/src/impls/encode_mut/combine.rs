use std::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};

pub struct Combine<EncoderA, EncoderB>(pub PhantomData<(EncoderA, EncoderB)>);

impl<Encoding, Strategy, EncoderA, EncoderB, Value> MutEncoder<Encoding, Strategy, Value>
    for Combine<EncoderA, EncoderB>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncoderA: MutEncoder<Encoding, Strategy, Value>,
    EncoderB: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncoderA::encode_mut(encoding, value, buffer)?;
        EncoderB::encode_mut(encoding, value, buffer)?;

        Ok(())
    }
}
