use cgp::prelude::*;
pub use hermes_cosmos_encoding_components::components::{
    CosmosEncodingComponents, DecoderComponent, EncodedLengthGetterComponent, EncoderComponent,
    MutDecoderComponent, MutEncoderComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_protobuf_encoding_components::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent, EncodedTypeComponent, SchemaTypeComponent,
};
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;

use crate::encoding::impls::client_state::EncodeStarknetClientState;
use crate::encoding::impls::consensus_state::EncodeStarknetConsensusState;
use crate::{StarknetClientState, StarknetConsensusState};

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
        ConverterComponent:
            DelegateEncoding<StarknetLightClientConverterComponents>,
    }
}

pub struct StarknetLightClientEncoderComponents;

pub struct StarknetLightClientEncodeMutComponents;

pub struct StarknetLightClientTypeUrlSchemas;

pub struct StarknetLightClientConverterComponents;

delegate_components! {
    StarknetLightClientEncoderComponents {
        [
            (ViaProtobuf, Any),
            (ViaProtobuf, Height),
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, StarknetConsensusState),
        ]: EncodeProtoWithMutBuffer,

        [
            (ViaAny, StarknetClientState),
            (ViaAny, StarknetConsensusState),
        ]: EncodeViaAny<ViaProtobuf>,
    }
}

delegate_components! {
    StarknetLightClientEncodeMutComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, Any),
            (ViaProtobuf, CommitmentRoot),
            (ViaProtobuf, Timestamp),
        ]: CosmosEncodingComponents,

        (ViaProtobuf, StarknetClientState):
            EncodeStarknetClientState,

        (ViaProtobuf, StarknetConsensusState):
            EncodeStarknetConsensusState,
    }
}

delegate_components! {
    StarknetLightClientConverterComponents {
        [
            (StarknetClientState, Any),
            (StarknetConsensusState, Any),
        ]: EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,

        [
            (Any, StarknetClientState),
            (Any, StarknetConsensusState),
        ]: DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
    }
}

delegate_components! {
    StarknetLightClientTypeUrlSchemas {
        StarknetClientState: StarknetClientStateUrl,
        StarknetConsensusState: StarknetConsensusStateUrl,
    }
}

impl_type_url!(StarknetClientStateUrl, "/StarknetClientState",);

impl_type_url!(StarknetConsensusStateUrl, "/StarknetConsensusState",);
