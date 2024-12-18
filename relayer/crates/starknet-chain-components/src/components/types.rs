use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    ChannelEndTypeComponent, ClientIdTypeComponent, ConnectionEndTypeComponent,
    ConnectionIdTypeComponent,
};

use crate::types::channel_end::ChannelEnd;
use crate::types::client_id::ClientId;
use crate::types::connection_id::{ConnectionEnd, ConnectionId};

cgp_preset! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
        ConnectionIdTypeComponent: ConnectionId,
        ConnectionEndTypeComponent: ConnectionEnd,
        ChannelEndTypeComponent: ChannelEnd,
    }
}
