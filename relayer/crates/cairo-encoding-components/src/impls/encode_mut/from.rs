use core::marker::PhantomData;

use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use hermes_prelude::*;

pub struct EncodeFrom<Intermediate, InEncoder>(pub PhantomData<(Intermediate, InEncoder)>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, Intermediate, InEncoder> MutEncoder<Encoding, Strategy, Value>
    for EncodeFrom<Intermediate, InEncoder>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    Intermediate: for<'a> From<&'a Value>,
    InEncoder: MutEncoder<Encoding, Strategy, Intermediate>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let intermediate = value.into();
        InEncoder::encode_mut(encoding, &intermediate, buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value, Intermediate, InEncoder> MutDecoder<Encoding, Strategy, Value>
    for EncodeFrom<Intermediate, InEncoder>
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<Intermediate::Error>,
    InEncoder: MutDecoder<Encoding, Strategy, Intermediate>,
    Intermediate: TryInto<Value>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Value, Encoding::Error> {
        let intermediate = InEncoder::decode_mut(encoding, buffer)?;
        let value = intermediate.try_into().map_err(Encoding::raise_error)?;

        Ok(value)
    }
}
