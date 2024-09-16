use cgp::prelude::*;
pub use hermes_cosmos_encoding_components::components::{
    CosmosEncodingComponents, DecoderComponent, EncodedLengthGetterComponent, EncoderComponent,
    MutDecoderComponent, MutEncoderComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_protobuf_encoding_components::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent, EncodedTypeComponent, SchemaTypeComponent,
};
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc_core::client::types::Height;

define_components! {
    StarknetLightClientEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            CosmosEncodingComponents,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<StarknetLightClientEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<StarknetLightClientEncodeMutComponents>,
    }
}

pub struct StarknetLightClientEncoderComponents;

pub struct StarknetLightClientEncodeMutComponents;

delegate_components! {
    StarknetLightClientEncoderComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosEncodingComponents,
    }
}

delegate_components! {
    StarknetLightClientEncodeMutComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosEncodingComponents,
    }
}
