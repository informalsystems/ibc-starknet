use cgp::core::Async;
use hermes_encoding_components::traits::types::encode_buffer::ProvideEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

pub struct ProvideVecEncodeBuffer;

impl<Encoding: Async> ProvideEncodeBufferType<Encoding> for ProvideVecEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>,
{
    type EncodeBuffer = Vec<Felt>;

    fn to_encoded(buffer: Vec<Felt>) -> Vec<Felt> {
        buffer
    }
}
