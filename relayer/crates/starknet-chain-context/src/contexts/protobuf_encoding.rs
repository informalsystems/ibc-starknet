use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::ViaProtobuf;
use hermes_starknet_chain_components::components::protobuf_encoding::*;
use hermes_starknet_chain_components::types::client::{
    StarknetClientState, StarknetConsensusState,
};

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
{
}

impl CanUseStarknetProtobufEncoding for StarknetProtobufEncoding {}
