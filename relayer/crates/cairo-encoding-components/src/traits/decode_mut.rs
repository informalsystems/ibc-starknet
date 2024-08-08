use cgp_core::prelude::*;
use hermes_encoding_components::traits::encoded::HasEncodedType;

#[derive_component(DecodeBufferTypeComponent, ProvideDecodeBufferType<Encoding>)]
pub trait HasDecodeBufferType: HasEncodedType {
    type DecodeBuffer;

    fn from_encoded(encoded: &Self::Encoded) -> Self::DecodeBuffer;
}

#[derive_component(MutDecoderComponent, MutDecoder<Encoding>)]
pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasErrorType {
    fn decode_mut(&self, buffer: &mut Self::DecodeBuffer) -> Result<Value, Self::Error>;
}
