use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use hermes_prelude::*;

use crate::impls::SumEncoders;
use crate::types::nat::Z;

pub struct EncodeEnumFields;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeEnumFields
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    Value: ToFieldsRef,
    SumEncoders<Z>: for<'a> MutEncoder<Encoding, Strategy, Value::FieldsRef<'a>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let fields = value.to_fields_ref();
        SumEncoders::encode_mut(encoding, &fields, buffer)
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeEnumFields
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    Value: FromFields,
    SumEncoders<Z>: MutDecoder<Encoding, Strategy, Value::Fields>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        let fields = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Value::from_fields(fields))
    }
}
