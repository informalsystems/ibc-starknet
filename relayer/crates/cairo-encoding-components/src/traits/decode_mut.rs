use cgp_core::prelude::*;

#[derive_component(DecodeBufferTypeComponent, ProvideDecodeBufferType<Encoding>)]
pub trait HasDecodeBufferType {
    type DecodeBuffer;
}

#[derive_component(MutDecoderComponent, MutDecoder<Encoding>)]
pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasErrorType {
    fn decode_mut(&self, buffer: &mut Self::DecodeBuffer) -> Result<Value, Self::Error>;
}
