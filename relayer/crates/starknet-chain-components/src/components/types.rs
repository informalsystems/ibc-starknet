use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::ClientIdTypeComponent;

use crate::types::client_id::ClientId;

define_components! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
    }
}
