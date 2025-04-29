use core::fmt::Display;
use core::str::FromStr;

use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;

pub struct EncodeDisplay;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeDisplay
where
    Encoding: CanEncodeMut<Strategy, String>,
    Value: Display,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.to_string(), buffer)
    }
}
#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeDisplay
where
    Encoding: CanDecodeMut<Strategy, String> + CanRaiseError<Value::Err>,
    Value: FromStr,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Value, Encoding::Error> {
        let value_str = encoding.decode_mut(buffer)?;

        let value = Value::from_str(&value_str).map_err(Encoding::raise_error)?;

        Ok(value)
    }
}
