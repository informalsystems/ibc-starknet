use hermes_encoding_components::traits::{
    EncodeBufferFinalizer, EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
    HasEncodeBufferType, HasEncodedType, ProvideEncodeBufferType,
};
use hermes_prelude::*;
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
