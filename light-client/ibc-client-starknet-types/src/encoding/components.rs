use cgp::prelude::*;
pub use hermes_cosmos_encoding_components::components::{
    CosmosEncodingComponents, DecoderComponent, EncodedLengthGetterComponent, EncoderComponent,
    MutDecoderComponent, MutEncoderComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_protobuf_encoding_components::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent, EncodedTypeComponent, SchemaTypeComponent,
};
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc_core::client::types::Height;

use crate::encoding::impls::client_state::EncodeStarknetClientState;
use crate::StarknetClientState;

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
        SchemaGetterComponent:
            DelegateEncoding<StarknetLightClientTypeUrlSchemas>,
    }
}

pub struct StarknetLightClientEncoderComponents;

pub struct StarknetLightClientEncodeMutComponents;

pub struct StarknetLightClientTypeUrlSchemas;

delegate_components! {
    StarknetLightClientEncoderComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, Any),
        ]: EncodeProtoWithMutBuffer,

        (ViaAny, StarknetClientState): EncodeViaAny<ViaProtobuf>,
    }
}

delegate_components! {
    StarknetLightClientEncodeMutComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, Any),
        ]: CosmosEncodingComponents,

        (ViaProtobuf, StarknetClientState):
            EncodeStarknetClientState,
    }
}

delegate_components! {
    StarknetLightClientTypeUrlSchemas {
        StarknetClientState: StarknetClientStateUrl,
    }
}

impl_type_url!(StarknetClientStateUrl, "/StarknetClientState",);
