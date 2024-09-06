use cgp::core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::{Any, ViaProtobuf};
use hermes_starknet_chain_components::components::encoding::protobuf::*;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::StarknetConsensusState;

use crate::impls::error::HandleStarknetError;

pub struct StarknetProtobufEncoding;

pub struct StarknetProtobufEncodingContextComponents;

impl HasComponents for StarknetProtobufEncoding {
    type Components = StarknetProtobufEncodingContextComponents;
}

delegate_components! {
    StarknetProtobufEncodingContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
    }
}

with_starknet_protobuf_encoding_components! {
    delegate_components! {
        StarknetProtobufEncodingContextComponents{
            @StarknetProtobufEncodingComponents: StarknetProtobufEncodingComponents,
        }
    }
}

pub trait CanUseStarknetProtobufEncoding:
    CanEncodeAndDecode<ViaProtobuf, StarknetClientState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetConsensusState>
    + CanConvert<StarknetClientState, Any>
    + CanConvert<Any, StarknetClientState>
    + CanConvert<StarknetConsensusState, Any>
    + CanConvert<Any, StarknetConsensusState>
    + CanConvert<WasmStarknetClientState, Any>
    + CanConvert<Any, WasmStarknetClientState>
{
}

impl CanUseStarknetProtobufEncoding for StarknetProtobufEncoding {}
