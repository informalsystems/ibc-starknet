use cgp_core::prelude::*;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;

use crate::impls::types::decode_buffer::ProvideVecIterDecodeBuffer;
use crate::impls::types::encode_buffer::ProvideVecEncodeBuffer;
use crate::impls::types::encoded::ProvideVecFeltEncodedType;
use crate::traits::decode_mut::DecodeBufferTypeComponent;
use crate::traits::encode_mut::EncodeBufferTypeComponent;

define_components! {
    CairoEncodingComponents {
        EncodedTypeComponent:
            ProvideVecFeltEncodedType,
        EncodeBufferTypeComponent:
            ProvideVecEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideVecIterDecodeBuffer,
    }
}
