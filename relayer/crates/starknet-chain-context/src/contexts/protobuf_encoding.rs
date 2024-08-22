use cgp_core::prelude::*;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_protobuf_encoding_components::types::ViaProtobuf;
use hermes_starknet_chain_components::components::protobuf_encoding::*;
use hermes_starknet_chain_components::types::client::{
    StarknetClientState, StarknetConsensusState,
};

pub struct StarknetProtobufEncoding;

pub struct StarknetProtobufEncodingContextComponents;

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
{
}
