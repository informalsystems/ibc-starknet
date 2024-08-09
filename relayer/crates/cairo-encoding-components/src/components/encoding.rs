use cgp_core::prelude::*;
pub use hermes_encoding_components::traits::decoder::DecoderComponent;
pub use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::encoder::EncoderComponent;

use crate::impls::encode::buffer::EncodeWithMutBuffer;
use crate::impls::types::decode_buffer::ProvideVecIterDecodeBuffer;
use crate::impls::types::encode_buffer::ProvideVecEncodeBuffer;
use crate::impls::types::encoded::ProvideVecFeltEncodedType;
pub use crate::traits::decode_mut::DecodeBufferTypeComponent;
pub use crate::traits::encode_mut::EncodeBufferTypeComponent;

define_components! {
    CairoEncodingComponents {
        EncodedTypeComponent:
            ProvideVecFeltEncodedType,
        EncodeBufferTypeComponent:
            ProvideVecEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideVecIterDecodeBuffer,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            EncodeWithMutBuffer,
    }
}
