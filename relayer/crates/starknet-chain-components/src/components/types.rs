use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::ClientIdTypeComponent;

use crate::types::client_id::ClientId;

cgp_preset! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
    }
}
