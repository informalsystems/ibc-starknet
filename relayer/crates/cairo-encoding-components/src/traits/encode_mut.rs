use cgp_core::prelude::*;

#[derive_component(EncodeBufferTypeComponent, ProvideEncodeBufferType<Encoding>)]
pub trait HasEncodeBufferType {
    type EncodeBuffer;
}

#[derive_component(MutEncoderComponent, MutEncoder<Encoding>)]
pub trait CanEncodeMut<Strategy, Value>: HasEncodeBufferType + HasErrorType {
    fn encode_mut(&self, value: &Value, buffer: &mut Self::EncodeBuffer)
        -> Result<(), Self::Error>;
}
