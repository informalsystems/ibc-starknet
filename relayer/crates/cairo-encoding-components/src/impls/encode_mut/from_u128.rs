use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};

pub struct EncodeFromU128;

#[cgp_provider(MutEncoderComponent)]
impl<Strategy, Encoding, Value, Error> MutEncoder<Encoding, Strategy, Value> for EncodeFromU128
where
    Encoding: CanEncodeMut<Strategy, u128> + CanRaiseAsyncError<Error>,
    Value: Clone + TryInto<u128, Error = Error>,
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
impl<Strategy, Encoding, Value, Error> MutDecoder<Encoding, Strategy, Value> for EncodeFromU128
where
    Encoding: CanDecodeMut<Strategy, u128> + CanRaiseAsyncError<Error>,
    Value: TryFrom<u128, Error = Error>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        let i = encoding.decode_mut(buffer)?;

        i.try_into().map_err(Encoding::raise_error)
    }
}
