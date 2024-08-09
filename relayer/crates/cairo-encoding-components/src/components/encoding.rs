use cgp_core::prelude::*;
pub use hermes_encoding_components::traits::encoded::EncodedTypeComponent;

use crate::components::encode_mut::CairoEncodeMutComponents;
use crate::impls::encode_mut::delegate::DelegateEncodeMutComponents;
use crate::impls::types::decode_buffer::ProvideVecIterDecodeBuffer;
use crate::impls::types::encode_buffer::ProvideVecEncodeBuffer;
use crate::impls::types::encoded::ProvideVecFeltEncodedType;
pub use crate::traits::decode_mut::{DecodeBufferTypeComponent, MutDecoderComponent};
pub use crate::traits::encode_mut::{EncodeBufferTypeComponent, MutEncoderComponent};

define_components! {
    CairoEncodingComponents {
        EncodedTypeComponent:
            ProvideVecFeltEncodedType,
        EncodeBufferTypeComponent:
            ProvideVecEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideVecIterDecodeBuffer,
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]:
            DelegateEncodeMutComponents<CairoEncodeMutComponents>,
    }
}
