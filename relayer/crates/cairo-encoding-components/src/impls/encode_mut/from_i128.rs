use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;

pub struct EncodeFromI128;

#[cgp_provider(MutEncoderComponent)]
impl<Strategy, Encoding, Value, Error> MutEncoder<Encoding, Strategy, Value> for EncodeFromI128
where
    Encoding: CanEncodeMut<Strategy, i128> + CanRaiseAsyncError<Error>,
    Value: Clone + TryInto<i128, Error = Error>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let i = value.clone().try_into().map_err(Encoding::raise_error)?;
        encoding.encode_mut(&i, buffer)
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Strategy, Encoding, Value, Error> MutDecoder<Encoding, Strategy, Value> for EncodeFromI128
where
    Encoding: CanDecodeMut<Strategy, i128> + CanRaiseAsyncError<Error>,
    Value: TryFrom<i128, Error = Error>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        let i = encoding.decode_mut(buffer)?;

        i.try_into().map_err(Encoding::raise_error)
    }
}
