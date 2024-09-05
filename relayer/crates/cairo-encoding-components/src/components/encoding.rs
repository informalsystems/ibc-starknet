use cgp::prelude::*;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::DecodeBufferPeekerComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;

use crate::impls::encode::buffer::EncodeWithMutBuffer;
use crate::impls::types::decode_buffer::ProvideVecIterDecodeBuffer;
use crate::impls::types::encode_buffer::ProvideVecEncodeBuffer;
use crate::impls::types::encoded::ProvideVecFeltEncodedType;

define_components! {
    CairoEncodingComponents {
        EncodedTypeComponent:
            ProvideVecFeltEncodedType,
        EncodeBufferTypeComponent:
            ProvideVecEncodeBuffer,
        [
            DecodeBufferTypeComponent,
            DecodeBufferPeekerComponent,
        ]:
            ProvideVecIterDecodeBuffer,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            EncodeWithMutBuffer,
    }
}
