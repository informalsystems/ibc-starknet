use cgp_core::prelude::*;
use hermes_encoding_components::traits::encoded::HasEncodedType;

#[derive_component(DecodeBufferTypeComponent, ProvideDecodeBufferType<Encoding>)]
pub trait HasDecodeBufferType: HasEncodedType {
    type DecodeBuffer<'a>;

    fn from_encoded<'a>(encoded: &'a Self::Encoded) -> Self::DecodeBuffer<'a>;
}

#[derive_component(MutDecoderComponent, MutDecoder<Encoding>)]
pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasErrorType {
    fn decode_mut(&self, buffer: &mut Self::DecodeBuffer<'_>) -> Result<Value, Self::Error>;
}
