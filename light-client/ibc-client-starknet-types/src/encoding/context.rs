use cgp::prelude::*;

use crate::encoding::components::*;

pub struct StarknetLightClientEncoding;

pub struct StarknetLightClientEncodingContextComponents;

with_starknet_light_client_encoding_components! {
    delegate_components! {
        StarknetLightClientEncodingContextComponents {
            @StarknetLightClientEncodingComponents: StarknetLightClientEncodingComponents
        }
    }
}
