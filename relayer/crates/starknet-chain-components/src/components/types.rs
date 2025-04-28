use cgp::prelude::*;
use hermes_chain_components::traits::{
    ChannelEndTypeComponent, ChannelIdTypeComponent, ClientIdTypeComponent,
    ConnectionEndTypeComponent, ConnectionIdTypeComponent,
};

use crate::types::channel_id::{ChannelEnd, ChannelId};
use crate::types::client_id::ClientId;
use crate::types::connection_id::{ConnectionEnd, ConnectionId};

pub struct StarknetChainTypes;

delegate_components! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
        ConnectionIdTypeComponent: ConnectionId,
        ChannelIdTypeComponent: ChannelId,
        ConnectionEndTypeComponent: ConnectionEnd,
        ChannelEndTypeComponent: ChannelEnd,
    }
}
