#[cgp::re_export_imports]
mod preset {
    use hermes_encoding_components::traits::{
        DecodeBufferBuilderComponent, DecodeBufferPeekerComponent, DecodeBufferTypeComponent,
        DecoderComponent, EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent,
    };
    use hermes_prelude::*;

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
