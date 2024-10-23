use cgp::core::component::{UseContext, UseDelegate};
use cgp::prelude::*;
pub use hermes_cosmos_chain_components::encoding::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
use hermes_protobuf_encoding_components::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::impls::convert::client_message::{
    DecodeViaClientMessage, EncodeViaClientMessage,
};
use hermes_wasm_encoding_components::types::client_message::WasmClientMessage;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::encoding::components::StarknetLightClientEncodingComponents;
use ibc_client_starknet_types::header::StarknetHeader;
use prost_types::Any;

use crate::types::client_state::{
    ConvertWasmStarknetClientState, StarknetClientState, WasmStarknetClientState,
};
use crate::types::consensus_state::{
    ConvertWasmStarknetConsensusState, StarknetConsensusState, WasmStarknetConsensusState,
};

define_components! {
    StarknetProtobufEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        EncodeBufferTypeComponent:
            ProvideBytesEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideProtoChunksDecodeBuffer,
        ConverterComponent:
            UseDelegate<StarknetConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            UseDelegate<StarknetEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            UseDelegate<StarknetMutEncoderComponents>,
        SchemaGetterComponent:
            UseDelegate<StarknetTypeUrlSchemas>,
    }
}

pub struct StarknetEncoderComponents;

pub struct StarknetMutEncoderComponents;

pub struct StarknetConverterComponents;

pub struct StarknetTypeUrlSchemas;

delegate_components! {
    StarknetEncoderComponents {
        [
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, StarknetConsensusState),
            (ViaProtobuf, StarknetHeader),

            (ViaAny, StarknetClientState),
            (ViaAny, StarknetConsensusState),
            (ViaAny, StarknetHeader),
        ]:
            StarknetLightClientEncodingComponents,

        [
            (ViaProtobuf, Any),

            (ViaAny, WasmClientState),
            (ViaProtobuf, WasmClientState),

            (ViaAny, WasmConsensusState),
            (ViaProtobuf, WasmConsensusState),

            (ViaAny, WasmClientMessage),
            (ViaProtobuf, WasmClientMessage),
        ]:
            WasmEncodingComponents,
    }
}

delegate_components! {
    StarknetMutEncoderComponents {
        [
            (ViaProtobuf, Height),
            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, WasmConsensusState),
            (ViaProtobuf, WasmClientMessage),
        ]: WasmEncodingComponents,

        [
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, StarknetConsensusState),
            (ViaProtobuf, StarknetHeader),
            (ViaProtobuf, CommitmentRoot),
            (ViaProtobuf, Timestamp),
        ]:
            StarknetLightClientEncodingComponents,
    }
}

delegate_components! {
    StarknetConverterComponents {
        (ClientMessage, Any): EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,
        (Any, ClientMessage): DecodeAsAnyProtobuf<ViaProtobuf, UseContext>,

        (StarknetHeader, Any):
            EncodeViaClientMessage,

        (Any, StarknetHeader):
            DecodeViaClientMessage,

        [
            (StarknetClientState, Any),
            (Any, StarknetClientState),
            (StarknetConsensusState, Any),
            (Any, StarknetConsensusState),
        ]:
            StarknetLightClientEncodingComponents,

        [
            (WasmClientState, Any),
            (Any, WasmClientState),
            (WasmConsensusState, Any),
            (Any, WasmConsensusState),
        ]:
            WasmEncodingComponents,

        [
            (Any, WasmStarknetClientState),
            (WasmStarknetClientState, Any),
        ]:
            ConvertWasmStarknetClientState,

        [
            (Any, WasmStarknetConsensusState),
            (WasmStarknetConsensusState, Any),
        ]:
            ConvertWasmStarknetConsensusState,
    }
}

delegate_components! {
    StarknetTypeUrlSchemas {
        ClientMessage: Self,

        [
            StarknetClientState,
            StarknetConsensusState,
            StarknetHeader,
        ]:
            StarknetLightClientEncodingComponents,

        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,
    }
}

impl_type_url!(
    StarknetTypeUrlSchemas,
    ClientMessage,
    "/ibc.lightclients.wasm.v1.ClientMessage",
);
