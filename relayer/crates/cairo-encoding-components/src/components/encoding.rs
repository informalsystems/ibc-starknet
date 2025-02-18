#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::DecodeBufferPeekerComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::types::decode_buffer::{
        DecodeBufferBuilderComponent, DecodeBufferTypeComponent,
    };
    use hermes_encoding_components::traits::types::encode_buffer::{
        EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
    };
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;

    use crate::impls::encode::buffer::EncodeWithMutBuffer;
    use crate::impls::types::decode_buffer::ProvideVecIterDecodeBuffer;
    use crate::impls::types::encode_buffer::ProvideVecEncodeBuffer;
    use crate::impls::types::encoded::ProvideVecFeltEncodedType;

    cgp_preset! {
        CairoEncodingComponents {
            EncodedTypeComponent:
                ProvideVecFeltEncodedType,
            [
                EncodeBufferTypeComponent,
                EncodeBufferFinalizerComponent,
            ]:
                ProvideVecEncodeBuffer,
            [
                DecodeBufferTypeComponent,
                DecodeBufferBuilderComponent,
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
}
