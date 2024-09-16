use cgp::core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_starknet_chain_components::components::encoding::protobuf::*;
use hermes_starknet_chain_components::types::client_header::StarknetClientHeader;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::{
    StarknetConsensusState, WasmStarknetConsensusState,
};
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::clients::wasm_types::client_message::ClientMessage;

use crate::impls::error::HandleStarknetChainError;

pub struct StarknetProtobufEncoding;

pub struct StarknetProtobufEncodingContextComponents;

impl HasComponents for StarknetProtobufEncoding {
    type Components = StarknetProtobufEncodingContextComponents;
}

delegate_components! {
    StarknetProtobufEncodingContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetChainError>,
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
    + CanEncodeAndDecode<ViaProtobuf, ClientMessage>
    + CanEncodeAndDecode<ViaProtobuf, WasmClientState>
    + CanEncodeAndDecode<ViaProtobuf, WasmConsensusState>
    + CanConvertBothWays<StarknetClientState, Any>
    + CanConvertBothWays<StarknetConsensusState, Any>
    + CanConvertBothWays<WasmStarknetClientState, Any>
    + CanConvertBothWays<StarknetClientHeader, Any>
    + CanConvertBothWays<WasmStarknetConsensusState, Any>
{
}

impl CanUseStarknetProtobufEncoding for StarknetProtobufEncoding {}
