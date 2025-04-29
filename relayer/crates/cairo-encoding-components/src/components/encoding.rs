#[cgp::re_export_imports]
mod preset {
    use hermes_encoding_components::traits::{
        DecodeBufferBuilderComponent, DecodeBufferPeekerComponent, DecodeBufferTypeComponent,
        DecoderComponent, EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent,
    };
    use hermes_prelude::*;

    use crate::impls::{
        EncodeWithMutBuffer, ProvideVecEncodeBuffer, ProvideVecFeltEncodedType,
        ProvideVecIterDecodeBuffer,
    };

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
