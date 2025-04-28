#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::{UseContext, UseDelegate};
    use cgp::prelude::*;
    use hermes_core::encoding_components::impls::{ProvideEncodedBytes, ProvideStringSchema};
    use hermes_core::encoding_components::traits::{
        ConverterComponent, DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };
    use hermes_cosmos_core::protobuf_encoding_components::impl_type_url;
    use hermes_cosmos_core::protobuf_encoding_components::impls::{
        DecodeAsAnyProtobuf, EncodeAsAnyProtobuf, ProvideBytesEncodeBuffer,
        ProvideProtoChunksDecodeBuffer,
    };
    use hermes_cosmos_core::protobuf_encoding_components::traits::EncodedLengthGetterComponent;
    use hermes_cosmos_core::protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
    use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
    use hermes_cosmos_core::wasm_encoding_components::impls::{
        DecodeViaClientMessage, EncodeViaClientMessage,
    };
    use hermes_cosmos_core::wasm_encoding_components::types::{
        WasmClientMessage, WasmClientState, WasmConsensusState,
    };
    use ibc::clients::wasm_types::client_message::ClientMessage;
    use ibc::core::client::types::Height;
    use ibc::core::commitment_types::commitment::CommitmentRoot;
    use ibc::primitives::Timestamp;
    use ibc_client_starknet_types::encoding::components::StarknetLightClientEncodingComponents;
    use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};
    use prost_types::Any;

    use crate::types::client_state::{
        ConvertWasmStarknetClientState, StarknetClientState, WasmStarknetClientState,
    };
    use crate::types::consensus_state::{
        ConvertWasmStarknetConsensusState, StarknetConsensusState, WasmStarknetConsensusState,
    };

    cgp_preset! {
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
                (ViaProtobuf, SignedStarknetHeader),

                (ViaAny, StarknetClientState),
                (ViaAny, StarknetConsensusState),
                (ViaAny, StarknetHeader),
                (ViaAny, SignedStarknetHeader),
            ]:
                StarknetLightClientEncodingComponents::Provider,

            [
                (ViaProtobuf, Any),

                (ViaAny, WasmClientState),
                (ViaProtobuf, WasmClientState),

                (ViaAny, WasmConsensusState),
                (ViaProtobuf, WasmConsensusState),

                (ViaAny, WasmClientMessage),
                (ViaProtobuf, WasmClientMessage),
            ]:
                WasmEncodingComponents::Provider,
        }
    }

    delegate_components! {
        StarknetMutEncoderComponents {
            [
                (ViaProtobuf, Height),
                (ViaProtobuf, WasmClientState),
                (ViaProtobuf, WasmConsensusState),
                (ViaProtobuf, WasmClientMessage),
            ]: WasmEncodingComponents::Provider,

            [
                (ViaProtobuf, StarknetClientState),
                (ViaProtobuf, StarknetConsensusState),
                (ViaProtobuf, StarknetHeader),
                (ViaProtobuf, SignedStarknetHeader),
                (ViaProtobuf, CommitmentRoot),
                (ViaProtobuf, Timestamp),
            ]:
                StarknetLightClientEncodingComponents::Provider,
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

            (SignedStarknetHeader, Any):
                EncodeViaClientMessage,

            (Any, SignedStarknetHeader):
                DecodeViaClientMessage,

            [
                (StarknetClientState, Any),
                (Any, StarknetClientState),
                (StarknetConsensusState, Any),
                (Any, StarknetConsensusState),
            ]:
                StarknetLightClientEncodingComponents::Provider,

            [
                (WasmClientState, Any),
                (Any, WasmClientState),
                (WasmConsensusState, Any),
                (Any, WasmConsensusState),
            ]:
                WasmEncodingComponents::Provider,

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
                SignedStarknetHeader,
            ]:
                StarknetLightClientEncodingComponents::Provider,

            [
                WasmClientState,
                WasmConsensusState,
            ]:
                WasmEncodingComponents::Provider,
        }
    }

    impl_type_url!(
        StarknetTypeUrlSchemas,
        ClientMessage,
        "/ibc.lightclients.wasm.v1.ClientMessage",
    );
}
