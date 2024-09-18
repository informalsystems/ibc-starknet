use cgp::prelude::*;
pub use hermes_cosmos_encoding_components::components::{
    CosmosEncodingComponents, DecoderComponent, EncodedLengthGetterComponent, EncoderComponent,
    MutDecoderComponent, MutEncoderComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::with_context::WithContext;
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
use crate::encoding::impls::header::EncodeStarknetHeader;
use crate::header::{StarknetHeader, STARKNET_HEADER_TYPE_URL};
use crate::{
    StarknetClientState, StarknetConsensusState, STARKNET_CLIENT_STATE_TYPE_URL,
    STARKNET_CONSENSUS_STATE_TYPE_URL,
};

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
            StarknetLightClientTypeUrlSchemas,
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
            (ViaProtobuf, StarknetHeader),
        ]: EncodeProtoWithMutBuffer,

        [
            (ViaAny, StarknetClientState),
            (ViaAny, StarknetConsensusState),
            (ViaAny, StarknetHeader),
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

        (ViaProtobuf, StarknetHeader):
            EncodeStarknetHeader,
    }
}

delegate_components! {
    StarknetLightClientConverterComponents {
        [
            (StarknetClientState, Any),
            (StarknetConsensusState, Any),
            (StarknetHeader, Any),
        ]: EncodeAsAnyProtobuf<ViaProtobuf, WithContext>,

        [
            (Any, StarknetClientState),
            (Any, StarknetConsensusState),
            (Any, StarknetHeader),
        ]: DecodeAsAnyProtobuf<ViaProtobuf, WithContext>,
    }
}

impl_type_url!(
    StarknetLightClientTypeUrlSchemas,
    StarknetClientState,
    STARKNET_CLIENT_STATE_TYPE_URL,
);

impl_type_url!(
    StarknetLightClientTypeUrlSchemas,
    StarknetConsensusState,
    STARKNET_CONSENSUS_STATE_TYPE_URL,
);

impl_type_url!(
    StarknetLightClientTypeUrlSchemas,
    StarknetHeader,
    STARKNET_HEADER_TYPE_URL,
);
