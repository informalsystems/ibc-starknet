use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_core::encoding_components::traits::{
    CanConvertBothWays, CanEncodeAndDecode, CanEncodeAndDecodeMut,
};
use hermes_cosmos::wasm_encoding_components::types::{WasmClientState, WasmConsensusState};
use hermes_error::impls::UseHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_starknet_chain_components::components::encoding::protobuf::*;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::{
    StarknetConsensusState, WasmStarknetConsensusState,
};
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};

use crate::impls::error::HandleStarknetChainError;

#[cgp_context(StarknetProtobufEncodingContextComponents: StarknetProtobufEncodingComponents)]
pub struct StarknetProtobufEncoding;

delegate_components! {
    StarknetProtobufEncodingContextComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
    }
}

pub trait CanUseStarknetProtobufEncoding:
    CanEncodeAndDecode<ViaProtobuf, StarknetClientState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, ClientMessage>
    + CanEncodeAndDecode<ViaProtobuf, WasmClientState>
    + CanEncodeAndDecode<ViaProtobuf, WasmConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetHeader>
    + CanEncodeAndDecode<ViaProtobuf, SignedStarknetHeader>
    + CanEncodeAndDecode<ViaAny, StarknetHeader>
    + CanConvertBothWays<StarknetClientState, Any>
    + CanConvertBothWays<StarknetConsensusState, Any>
    + CanConvertBothWays<WasmStarknetClientState, Any>
    + CanConvertBothWays<WasmStarknetConsensusState, Any>
    + CanConvertBothWays<StarknetHeader, Any>
    + CanEncodeAndDecodeMut<ViaProtobuf, Timestamp>
    + CanEncodeAndDecodeMut<ViaProtobuf, CommitmentRoot>
{
}

impl CanUseStarknetProtobufEncoding for StarknetProtobufEncoding {}
