use cgp::prelude::*;
pub use hermes_cosmos_chain_components::encoding::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encode::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
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
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
use hermes_protobuf_encoding_components::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::encoding::components::StarknetLightClientEncodingComponents;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as ProtoClientMessage;
use prost_types::Any;

use crate::types::client_header::{ConvertStarknetClientHeader, StarknetClientHeader};
use crate::types::client_state::{
    ConvertWasmStarknetClientState, StarknetClientState,
    WasmStarknetClientState,
};
use crate::types::consensus_state::{
    ConvertWasmStarknetConsensusState, StarknetConsensusState,
    WasmStarknetConsensusState,
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
            DelegateEncoding<StarknetConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<StarknetEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<StarknetMutEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<StarknetTypeUrlSchemas>,
    }
}

pub struct StarknetEncoderComponents;

pub struct StarknetMutEncoderComponents;

pub struct StarknetConverterComponents;

pub struct StarknetTypeUrlSchemas;

delegate_components! {
    StarknetEncoderComponents {
        (ViaAny, ClientMessage): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf, ClientMessage): ConvertAndEncode<ProtoClientMessage>,
        (ViaProtobuf, ProtoClientMessage): EncodeAsProtobuf,

        [
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, StarknetConsensusState),

            (ViaAny, StarknetClientState),
            (ViaAny, StarknetConsensusState),
        ]:
            StarknetLightClientEncodingComponents,

        [
            (ViaProtobuf, Any),
            (ViaAny, WasmClientState),
            (ViaProtobuf, WasmClientState),
            (ViaAny, WasmConsensusState),
            (ViaProtobuf, WasmConsensusState),
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
        ]: WasmEncodingComponents,

        [
            (ViaProtobuf, StarknetClientState),
            (ViaProtobuf, StarknetConsensusState),
            (ViaProtobuf, CommitmentRoot),
            (ViaProtobuf, Timestamp),
        ]:
            StarknetLightClientEncodingComponents,
    }
}

delegate_components! {
    StarknetConverterComponents {
        (ClientMessage, ProtoClientMessage): ConvertFrom,
        (ProtoClientMessage, ClientMessage): TryConvertFrom,

        (ClientMessage, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, ClientMessage): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,

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

        [
            (Any, StarknetClientHeader),
            (StarknetClientHeader, Any),
        ]:
            ConvertStarknetClientHeader,

    }
}

delegate_components! {
    StarknetTypeUrlSchemas {
        ClientMessage: ClientMessageUrl,

        [
            StarknetClientState,
            StarknetConsensusState,
        ]:
            StarknetLightClientEncodingComponents,

        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,
    }
}

impl_type_url!(ClientMessageUrl, "/ibc.lightclients.wasm.v1.ClientMessage");
