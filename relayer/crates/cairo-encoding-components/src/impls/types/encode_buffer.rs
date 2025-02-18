use cgp::prelude::*;
use hermes_encoding_components::traits::types::encode_buffer::{
    EncodeBufferFinalizer, EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
    HasEncodeBufferType, ProvideEncodeBufferType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

pub struct ProvideVecEncodeBuffer;

#[cgp_provider(EncodeBufferTypeComponent)]
impl<Encoding> ProvideEncodeBufferType<Encoding> for ProvideVecEncodeBuffer {
    type EncodeBuffer = Vec<Felt>;
}

#[cgp_provider(EncodeBufferFinalizerComponent)]
impl<Encoding> EncodeBufferFinalizer<Encoding> for ProvideVecEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + HasEncodeBufferType<EncodeBuffer = Vec<Felt>>,
{
    fn to_encoded(buffer: Vec<Felt>) -> Vec<Felt> {
        buffer
    }
}
