#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::{UseContext, UseDelegate};
    use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
    use hermes_encoding_components::traits::{
        ConverterComponent, DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };
    use hermes_prelude::*;
    use hermes_protobuf_encoding_components::impl_type_url;
    use hermes_protobuf_encoding_components::impls::{
        DecodeAsAnyProtobuf, EncodeAsAnyProtobuf, EncodeProtoWithMutBuffer, EncodeViaAny,
    };
    use hermes_protobuf_encoding_components::traits::EncodedLengthGetterComponent;
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

    cgp_preset! {
        StarknetLightClientEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                DecodeBufferTypeComponent,
                SchemaTypeComponent,
            ]:
                CosmosEncodingComponents::Provider,
            [
                EncoderComponent,
                DecoderComponent,
            ]:
                UseDelegate<StarknetLightClientEncoderComponents>,
            [
                MutEncoderComponent,
                MutDecoderComponent,
                EncodedLengthGetterComponent,
            ]:
                UseDelegate<StarknetLightClientEncodeMutComponents>,
            SchemaGetterComponent:
                StarknetLightClientTypeUrlSchemas,
            ConverterComponent:
                UseDelegate<StarknetLightClientConverterComponents>,
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
            ]: CosmosEncodingComponents::Provider,

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
            ]: EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,

            [
                (Any, StarknetClientState),
                (Any, StarknetConsensusState),
                (Any, StarknetHeader),
            ]: DecodeAsAnyProtobuf<ViaProtobuf, UseContext>,
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
}
