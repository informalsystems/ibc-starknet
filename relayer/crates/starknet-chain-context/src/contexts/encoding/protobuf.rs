use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encode_and_decode_mut::CanEncodeAndDecodeMut;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_starknet_chain_components::components::encoding::protobuf::*;
use hermes_starknet_chain_components::types::client_state::{
    StarknetClientState, WasmStarknetClientState,
};
use hermes_starknet_chain_components::types::consensus_state::{
    StarknetConsensusState, WasmStarknetConsensusState,
};
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;

use crate::impls::error::HandleStarknetChainError;

pub struct StarknetProtobufEncoding;

pub struct StarknetProtobufEncodingContextComponents;

impl HasComponents for StarknetProtobufEncoding {
    type Components = StarknetProtobufEncodingContextComponents;
}

delegate_components! {
    StarknetProtobufEncodingContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
    }
}

with_starknet_protobuf_encoding_components! {
    | Components | {
        delegate_components! {
            StarknetProtobufEncodingContextComponents{
                Components: StarknetProtobufEncodingComponents,
            }
        }
    }
}

pub trait CanUseStarknetProtobufEncoding:
    CanEncodeAndDecode<ViaProtobuf, StarknetClientState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, ClientMessage>
    + CanEncodeAndDecode<ViaProtobuf, WasmClientState>
    + CanEncodeAndDecode<ViaProtobuf, WasmConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetHeader>
    + CanEncodeAndDecode<ViaAny, StarknetHeader>
    + CanConvertBothWays<StarknetClientState, Any>
    + CanConvertBothWays<StarknetConsensusState, Any>
    + CanConvertBothWays<WasmStarknetClientState, Any>
    + CanConvertBothWays<WasmStarknetConsensusState, Any>
    + CanConvertBothWays<StarknetHeader, Any>
    + CanEncodeAndDecodeMut<ViaProtobuf, Timestamp>
    + CanEncodeAndDecodeMut<ViaProtobuf, CommitmentRoot>
// + CanEncodeAndDecodeMut<ViaProtobuf, ChainId>
{
}

impl CanUseStarknetProtobufEncoding for StarknetProtobufEncoding {}
