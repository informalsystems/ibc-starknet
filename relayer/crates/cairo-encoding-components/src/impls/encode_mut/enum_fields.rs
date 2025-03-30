use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{MutDecoder, MutDecoderComponent};
use hermes_encoding_components::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::size::HasSize;
use crate::types::nat::{S, Z};

pub struct EncodeEnumFields;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, N> MutEncoder<Encoding, Strategy, Value> for EncodeEnumFields
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    Value: ToFieldsRef,
    SumEncoders<Z, N>: for<'a> MutEncoder<Encoding, Strategy, Value::FieldsRef<'a>>,
    for<'a> Value::FieldsRef<'a>: HasSize<Size = S<N>>,
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
impl<Encoding, Strategy, Value, N> MutDecoder<Encoding, Strategy, Value> for EncodeEnumFields
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    Value: FromFields,
    SumEncoders<Z, N>: MutDecoder<Encoding, Strategy, Value::Fields>,
    Value::Fields: HasSize<Size = S<N>>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        let fields = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Value::from_fields(fields))
    }
}
